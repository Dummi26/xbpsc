# xbpsc

A wrapper for [the xbps package manager](https://docs.voidlinux.org/xbps/index.html).

This program always run `xbps-$@`: `xbpsc install -Su` becomes `xbps-install -Su`.

While I try to stay true to the original output from xbps,
I will rephrase and reformat things to make them be what I would like.

I also want to add things like proper progress bars, maybe an optional tui-style interface
(scrollable package lists, ... - but in a different executable, maybe xbpstui?) etc to make things more readable or just more elegant.

Any command that works with xbps-whatever will also work from xbpsc - there will be no new syntax to learn and no weird pitfalls to cause havoc on your system.

I also aim to keep the codebase as small as I can so people can read through it all before running this as root (which is obviously necessary for xbps).

## Colors

- errors are red
- Warnings are yellow
- anything from xbps's stderr is cyan (this includes the [Y/n] prompts)
- steps and progress updates are blue
- custom user-related things are purple (such as the user's inputs, package names, etc.)
  + usually, red/yellow will be used to describe a problem and purple will be used to indicate the cause of that problem
- if the meaning of a line can't be determined, it will also be purple.

