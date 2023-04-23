mod line;

use std::{
    io::{BufRead, BufReader, Read},
    process::{Command, Stdio},
    time::Duration,
};

use regex_macro::regex;

use crate::line::Line;

trait CapturesOne {
    fn cpt<'a>(&self, text: &'a str) -> Option<&'a str>;
}
impl CapturesOne for regex::Regex {
    fn cpt<'a>(&self, text: &'a str) -> Option<&'a str> {
        if let Some(captures) = self.captures(text) {
            if let Some(capture) = captures.get(1) {
                return Some(capture.as_str());
            }
        }
        None
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    if let Some(xbps_what) = args.next() {
        if let Ok(mut xbps) = Command::new(format!("xbps-{xbps_what}").as_str())
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            if let (Some(stdout), Some(stderr)) = (xbps.stdout.take(), xbps.stderr.take()) {
                let (line_sender, line_receiver) = std::sync::mpsc::channel();
                let stdout_thread = {
                    let line_sender: std::sync::mpsc::Sender<(bool, String)> = line_sender.clone();
                    std::thread::spawn(move || {
                        let mut stdout = BufReader::new(stdout);
                        let mut read = String::new();
                        loop {
                            match stdout.read_line(&mut read) {
                                Ok(0) => return,
                                Ok(_) => {
                                    // print!("STDOUT: {read}");
                                    line_sender
                                        .send((false, read[0..read.len() - 1].to_owned()))
                                        .unwrap();
                                    read.clear();
                                }
                                Err(_) => return,
                            }
                        }
                    })
                    // THIS IS A COPY OF THE CODE FROM STDERR_THREAD, WHICH CAN READ STUFF THAT ISN'T NEWLINE TERMINATED
                    // i'm not using it because it didn't help with the problems I was having so it's just unnecessary.
                    // let line_sender = line_sender.clone();
                    // std::thread::spawn(move || -> Result<(), std::io::Error> {
                    //     let mut stdout = BufReader::new(stdout);
                    //     let mut buf = [0];
                    //     let mut bytes = vec![];
                    //     loop {
                    //         stdout.read_exact(&mut buf)?;
                    //         bytes.push(buf[0]);
                    //         if let Ok(str) = std::str::from_utf8(&bytes) {
                    //             eprintln!("STDOUT: {}", str);
                    //             'send: {
                    //                 line_sender
                    //                     .send((
                    //                         true,
                    //                         if str.ends_with("\n") {
                    //                             let o = str[0..str.len() - 1].to_owned();
                    //                             bytes.clear();
                    //                             o
                    //                         } else if str.ends_with("[Y/n]")
                    //                             || str.ends_with("[y/N]")
                    //                         {
                    //                             str.to_owned()
                    //                         } else {
                    //                             break 'send;
                    //                         },
                    //                     ))
                    //                     .unwrap();
                    //             };
                    //         }
                    //     }
                    // })
                };
                let stderr_thread = {
                    let line_sender = line_sender.clone();
                    std::thread::spawn(move || -> Result<(), std::io::Error> {
                        let mut stderr = BufReader::new(stderr);
                        let mut buf = [0];
                        let mut bytes = vec![];
                        loop {
                            stderr.read_exact(&mut buf)?;
                            bytes.push(buf[0]);
                            if let Ok(str) = std::str::from_utf8(&bytes) {
                                // eprintln!("STDERR: {}", str);
                                'send: {
                                    line_sender
                                        .send((
                                            true,
                                            if str.ends_with("\n") {
                                                let o = str[0..str.len() - 1].to_owned();
                                                bytes.clear();
                                                o
                                            } else if str.ends_with("[Y/n]")
                                                || str.ends_with("[y/N]")
                                            {
                                                str.to_owned()
                                            } else {
                                                break 'send;
                                            },
                                        ))
                                        .unwrap();
                                };
                            }
                        }
                    })
                };
                let mut line_receiver = LineReceiver(line_receiver);
                'outer: loop {
                    let mut got_nothing = 0;
                    let mut prev_lines = Line::None;
                    loop {
                        let mut got_something = false;
                        for (was_stderr, line) in &mut line_receiver {
                            got_something = true;
                            // eprintln!(
                            //     "Received line on {}: {}",
                            //     if was_stderr { "stderr" } else { "stdout" },
                            //     line
                            // );
                            let line = if !was_stderr {
                                if let Some(package) =
                                    regex!(r"Package `(.*)' already installed\.").cpt(&line)
                                {
                                    Line::PackageAlreadyInstalled(vec![package.to_owned()])
                                } else if line.starts_with("[*] ") {
                                    Line::StepLine(line[4..].to_owned())
                                } else {
                                    Line::Other(line)
                                }
                            } else {
                                Line::OtherStderr(line)
                            };
                            prev_lines.push(line);
                            got_nothing = 0;
                        }
                        // if there are no new lines for a period of time, stop waiting for more lines and print the current prev_lines.
                        // otherwise, we might wait for a different type of line to show up before printing one, which would be annoying
                        // in cases where there is a prolonged silence on stdout/stderr.
                        if !got_something {
                            if got_nothing > 10 {
                                // no new lines were read
                                if stderr_thread.is_finished() || stdout_thread.is_finished() {
                                    prev_lines.push(Line::None);
                                    // if stderr_thread.is_finished() {
                                    //     if stdout_thread.is_finished() {
                                    //         println!("Exiting: No more stdio.");
                                    //     } else {
                                    //         println!("Exiting: No more stderr.");
                                    //     }
                                    // } else {
                                    //     println!("Exiting: No more stdout.");
                                    // }
                                    break 'outer;
                                } else {
                                    break;
                                }
                            }
                            got_nothing += 1;
                            std::thread::sleep(Duration::from_millis(10));
                        }
                    }
                    // print the last line
                    prev_lines.push(Line::None);
                }
                // eprintln!("goodbye");
            }
        }
    }
}

struct LineReceiver(std::sync::mpsc::Receiver<(bool, String)>);
impl Iterator for LineReceiver {
    type Item = (bool, String);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.try_recv().ok()
    }
}
