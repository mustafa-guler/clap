Git is an example of several common subcommand patterns.

Help:
```console
$ git
? failed
A fictional versioning CLI

Usage: git[EXE] <COMMAND>

Commands:
    clone    Clones repos
    diff     Compare two commits
    push     pushes things
    add      adds things
    stash    
    help     Print this message or the help of the given subcommand(s)

Options:
    -h, --help    Print help information

$ git help
A fictional versioning CLI

Usage: git[EXE] <COMMAND>

Commands:
    clone    Clones repos
    diff     Compare two commits
    push     pushes things
    add      adds things
    stash    
    help     Print this message or the help of the given subcommand(s)

Options:
    -h, --help    Print help information

$ git help add
adds things

Usage: git[EXE] add <PATH>...

Arguments:
    <PATH>...    Stuff to add

Options:
    -h, --help    Print help information

```

A basic argument:
```console
$ git add
? failed
adds things

Usage: git[EXE] add <PATH>...

Arguments:
    <PATH>...    Stuff to add

Options:
    -h, --help    Print help information

$ git add Cargo.toml Cargo.lock
Adding ["Cargo.toml", "Cargo.lock"]

```

Default subcommand:
```console
$ git stash -h
Usage: git[EXE] stash [OPTIONS]
       git[EXE] stash <COMMAND>

Commands:
    push     
    pop      
    apply    
    help     Print this message or the help of the given subcommand(s)

Options:
    -m, --message <MESSAGE>    
    -h, --help                 Print help information

$ git stash push -h
Usage: git[EXE] stash push [OPTIONS]

Options:
    -m, --message <MESSAGE>    
    -h, --help                 Print help information

$ git stash pop -h
Usage: git[EXE] stash pop [STASH]

Arguments:
    [STASH]    

Options:
    -h, --help    Print help information

$ git stash -m "Prototype"
Pushing Some("Prototype")

$ git stash pop
Popping None

$ git stash push -m "Prototype"
Pushing Some("Prototype")

$ git stash pop
Popping None

```

External subcommands:
```console
$ git custom-tool arg1 --foo bar
Calling out to "custom-tool" with ["arg1", "--foo", "bar"]

```

Last argument:
```console
$ git diff --help
Compare two commits

Usage: git[EXE] diff [COMMIT] [COMMIT] [-- <PATH>]

Arguments:
    [COMMIT]    
    [COMMIT]    
    [PATH]      

Options:
    -h, --help    Print help information

$ git diff
Diffing stage..worktree 

$ git diff ./src
Diffing stage..worktree ./src

$ git diff HEAD ./src
Diffing HEAD..worktree ./src

$ git diff HEAD~~ -- HEAD
Diffing HEAD~~..worktree HEAD

```
