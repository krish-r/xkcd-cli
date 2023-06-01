use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn insta_test_help_message() -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::cargo_bin("xkcd-cli")?;
    command.arg("--help");

    let assert = command.assert().success();
    let output = std::str::from_utf8(&assert.get_output().stdout)?;

    insta::assert_snapshot!(&output, @r###"
    Usage: xkcd-cli [OPTIONS]

    Options:
      -l, --latest           latest
      -r, --random           random
      -o, --output <OUTPUT>  output
      -h, --help             Print help
    "###);

    Ok(())
}

#[test]
fn return_error_for_an_invalid_flag() -> Result<(), Box<dyn std::error::Error>> {
    let err_msg = r###"error: unexpected argument '--invalid' found

Usage: xkcd-cli [OPTIONS]

For more information, try '--help'.
"###;

    let mut command = Command::cargo_bin("xkcd-cli")?;
    command.arg("--invalid");
    command
        .assert()
        .failure()
        .stderr(predicate::str::contains(err_msg));

    Ok(())
}

#[test]
fn return_error_for_no_flag() -> Result<(), Box<dyn std::error::Error>> {
    let err_msg = r###"Expecting at least one argument. Check `--help`"###;

    let mut command = Command::cargo_bin("xkcd-cli")?;
    command
        .assert()
        .failure()
        .stderr(predicate::str::contains(err_msg));

    Ok(())
}

#[test]
fn return_error_for_an_multiple_flags_from_arg_group() -> Result<(), Box<dyn std::error::Error>> {
    let err_msg = r###"error: the argument '--latest' cannot be used with '--random'

Usage: xkcd-cli --latest

For more information, try '--help'.
"###;

    let mut command = Command::cargo_bin("xkcd-cli")?;
    command.arg("--latest").arg("--random");
    command
        .assert()
        .failure()
        .stderr(predicate::str::contains(err_msg));

    Ok(())
}
