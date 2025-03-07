use super::utils;

use clap::{arg, error::ErrorKind, value_parser, Arg, Command, Error};

#[track_caller]
fn assert_error<F: clap::error::ErrorFormatter>(
    err: Error<F>,
    expected_kind: ErrorKind,
    expected_output: &str,
    stderr: bool,
) {
    let actual_output = err.to_string();
    assert_eq!(
        stderr,
        err.use_stderr(),
        "Should Use STDERR failed. Should be {} but is {}",
        stderr,
        err.use_stderr()
    );
    assert_eq!(expected_kind, err.kind());
    utils::assert_eq(expected_output, actual_output)
}

#[test]
fn app_error() {
    static MESSAGE: &str = "error: Failed for mysterious reasons

Usage: test [OPTIONS] --all

For more information try --help
";
    let cmd = Command::new("test")
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .required(true)
                .action(clap::ArgAction::SetTrue)
                .help("Also do versioning for private crates (will not be published)"),
        )
        .arg(
            Arg::new("exact")
                .long("exact")
                .help("Specify inter dependency version numbers exactly with `=`"),
        )
        .arg(
            Arg::new("no_git_commit")
                .long("no-git-commit")
                .help("Do not commit version changes"),
        )
        .arg(
            Arg::new("no_git_push")
                .long("no-git-push")
                .help("Do not push generated commit and tags to git remote"),
        );
    let mut cmd = cmd;
    let expected_kind = ErrorKind::InvalidValue;
    let err = cmd.error(expected_kind, "Failed for mysterious reasons");
    assert_error(err, expected_kind, MESSAGE, true);
}

#[test]
fn value_validation_has_newline() {
    let res = Command::new("test")
        .arg(
            arg!(<PORT>)
                .value_parser(value_parser!(usize))
                .help("Network port to use"),
        )
        .try_get_matches_from(["test", "foo"]);

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert!(
        err.to_string().ends_with('\n'),
        "Errors should have a trailing newline, got {:?}",
        err.to_string()
    );
}

#[test]
fn null_prints_help() {
    let cmd = Command::new("test");
    let res = cmd
        .try_get_matches_from(["test", "--help"])
        .map_err(|e| e.apply::<clap::error::NullFormatter>());
    assert!(res.is_err());
    let err = res.unwrap_err();
    let expected_kind = ErrorKind::DisplayHelp;
    static MESSAGE: &str = "\
Usage: test

Options:
    -h, --help    Print help information
";
    assert_error(err, expected_kind, MESSAGE, false);
}

#[test]
fn raw_prints_help() {
    let cmd = Command::new("test");
    let res = cmd
        .try_get_matches_from(["test", "--help"])
        .map_err(|e| e.apply::<clap::error::RawFormatter>());
    assert!(res.is_err());
    let err = res.unwrap_err();
    let expected_kind = ErrorKind::DisplayHelp;
    static MESSAGE: &str = "\
Usage: test

Options:
    -h, --help    Print help information
";
    assert_error(err, expected_kind, MESSAGE, false);
}

#[test]
fn null_ignores_validation_error() {
    let cmd = Command::new("test");
    let res = cmd
        .try_get_matches_from(["test", "unused"])
        .map_err(|e| e.apply::<clap::error::NullFormatter>());
    assert!(res.is_err());
    let err = res.unwrap_err();
    let expected_kind = ErrorKind::UnknownArgument;
    static MESSAGE: &str = "";
    assert_error(err, expected_kind, MESSAGE, true);
}

#[test]
fn rich_formats_validation_error() {
    let cmd = Command::new("test");
    let res = cmd.try_get_matches_from(["test", "unused"]);
    assert!(res.is_err());
    let err = res.unwrap_err();
    let expected_kind = ErrorKind::UnknownArgument;
    static MESSAGE: &str = "\
error: Found argument 'unused' which wasn't expected, or isn't valid in this context

Usage: test

For more information try --help
";
    assert_error(err, expected_kind, MESSAGE, true);
}

#[test]
fn raw_formats_validation_error() {
    let cmd = Command::new("test");
    let res = cmd
        .try_get_matches_from(["test", "unused"])
        .map_err(|e| e.apply::<clap::error::RawFormatter>());
    assert!(res.is_err());
    let err = res.unwrap_err();
    let expected_kind = ErrorKind::UnknownArgument;
    static MESSAGE: &str = "\
error: Found an argument which wasn't expected or isn't valid in this context

Invalid Argument: unused
Usage: test
";
    assert_error(err, expected_kind, MESSAGE, true);
}
