```console
$ 03_01_flag_bool_derive --help
A simple to use, efficient, and full-featured Command Line Argument Parser

Usage: 03_01_flag_bool_derive[EXE] [OPTIONS]

Options:
    -v, --verbose    
    -h, --help       Print help information
    -V, --version    Print version information

$ 03_01_flag_bool_derive
verbose: false

$ 03_01_flag_bool_derive --verbose
verbose: true

$ 03_01_flag_bool_derive --verbose --verbose
verbose: true

```
