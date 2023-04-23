use std::fmt::Display;

use colored::Colorize;

#[repr(u8)]
pub enum Line {
    PackageAlreadyInstalled(Vec<String>),
    StepLine(String),
    Other(String),
    OtherStderr(String),
    None,
}
impl Line {
    pub fn push(&mut self, v: Self) {
        match (self, v) {
            (_, Self::OtherStderr(v)) => eprintln!("{}", v.cyan()),
            (Self::PackageAlreadyInstalled(a), Self::PackageAlreadyInstalled(mut b)) => {
                a.append(&mut b)
            }
            (s, v) => {
                if let Self::None = s {
                } else {
                    println!("{s}");
                }
                *s = v;
            }
        }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PackageAlreadyInstalled(packages) => write!(
                f,
                "{}{}{}{}",
                if packages.len() == 1 {
                    "Package "
                } else {
                    "Packages "
                }
                .yellow(),
                {
                    if packages.len() == 1 {
                        format!("'{}'", &packages[0].purple())
                    } else if packages.len() == 2 {
                        format!(
                            "'{}' and '{}'",
                            &packages[0].purple(),
                            &packages[1].purple()
                        )
                    } else {
                        let mut text = String::with_capacity(
                            packages.iter().fold(0, |len, v| len + v.len() + 6) + 3,
                        );
                        for (i, package) in packages.iter().enumerate() {
                            if i + 1 == packages.len() {
                                text.push_str(" and ");
                                text.push_str(&package.purple().to_string());
                            } else {
                                text.push_str(&package.purple().to_string());
                                if i + 2 != packages.len() {
                                    text.push_str(", ");
                                }
                            }
                        }
                        text
                    }
                },
                if packages.len() == 1 { " is " } else { " are " }.yellow(),
                "already installed.".yellow(),
            ),
            Self::StepLine(step) => write!(f, "[*] {}", step.blue()),
            Self::Other(line) => write!(f, "{}", line.purple()),
            Self::OtherStderr(_line) => Ok(()),
            Self::None => Ok(()),
        }
    }
}
