```console
$ 04_02_validate --help
A simple to use, efficient, and full-featured Command Line Argument Parser

Usage: 04_02_validate[EXE] <PORT>

Arguments:
    <PORT>    Network port to use

Options:
    -h, --help       Print help information
    -V, --version    Print version information

$ 04_02_validate 22
PORT = 22

$ 04_02_validate foobar
? failed
error: Invalid value "foobar" for '<PORT>': `foobar` isn't a port number

For more information try --help

$ 04_02_validate 0
? failed
error: Invalid value "0" for '<PORT>': Port not in range 1-65535

For more information try --help

```
