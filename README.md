# xbpsc

A wrapper for [the xbps package manager](https://docs.voidlinux.org/xbps/index.html).

This program always run `xbps-$@`: `xbpsc install -Su` becomes `xbps-install -Su`.

Anything that works with xbps-whatever commands should be doable from xbpsc too.

## Colors

- errors are red
- Warnings are yellow
- anything from xbps's stderr is cyan (this includes the [Y/n] prompts)
- steps and progress updates are blue
- custom user-related things are purple (such as the user's inputs, package names, etc.)
  + usually, red/yellow will be used to describe a problem and purple will be used to indicate the cause of that problem
- if the meaning of a line can't be determined, it will also be purple.

