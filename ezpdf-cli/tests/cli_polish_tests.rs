use assert_cmd::Command;
use predicates::prelude::PredicateBooleanExt;

#[test]
fn version_exits_zero_and_contains_version_string() {
    Command::cargo_bin("ezpdf")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicates::str::contains("0.1.0"));
}

#[test]
fn unknown_subcommand_exits_nonzero_with_error_message() {
    Command::cargo_bin("ezpdf")
        .unwrap()
        .arg("nonexistent_command")
        .assert()
        .failure()
        .stderr(predicates::str::contains("error"));
}

#[test]
fn completions_zsh_exits_zero_and_produces_output() {
    Command::cargo_bin("ezpdf")
        .unwrap()
        .args(["completions", "zsh"])
        .assert()
        .success()
        .stdout(predicates::str::is_empty().not());
}

#[test]
fn completions_bash_exits_zero_and_produces_output() {
    Command::cargo_bin("ezpdf")
        .unwrap()
        .args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicates::str::is_empty().not());
}
