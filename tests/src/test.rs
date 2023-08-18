use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use toml;

const BIN_NAME: &'static str = "noj_sandbox";

fn common_compile_args() -> &'static [&'static str] {
    &["-DONLINE_JUDGE", "-O2", "-w", "-fmax-errors=3"]
}

#[test]
fn test_help() {
    let mut cmd = assert_cmd::Command::cargo_bin(BIN_NAME).unwrap();
    let predicate_help_message = predicates::str::starts_with(format!("Usage: {BIN_NAME}"));
    cmd.arg("-h")
        .assert()
        .stdout(predicate_help_message.clone())
        .success();
    cmd.arg("--help")
        .assert()
        .stdout(predicate_help_message)
        .success();
}

#[test]
fn test_execute_c_code() {
    let test_root = assert_fs::TempDir::new().unwrap();
    test_root
        .child("main.c")
        .write_str(
            r#"
#include <stdio.h>

int main()
{
    printf("hello, world!\n");
    return 0;
}
"#,
        )
        .unwrap();
    test_root.child("stdin").touch().unwrap();

    let mut compile_cmd = assert_cmd::Command::new("gcc");
    compile_cmd
        .args(common_compile_args())
        .args(["-std=c++11", "main.c", "-o", "main", "-lm"])
        .current_dir(test_root.path())
        .assert()
        .success();

    let mut config = toml::toml! {
        cwd = "./"
        large-stack = true
        max-process = 10
        memory-limit = 5120000
        output-size-limit = 10000
        runtime-limit = 1500
        stdin = "./stdin"
        stdout = "./stdout"
        lang = 1
    };
    for f in ["stdin", "stdout", "stderr", "output"] {
        config.insert(
            f.to_string(),
            toml::Value::String(test_root.path().join(f).to_str().unwrap().to_owned()),
        );
    }
    test_root
        .child("config.toml")
        .write_str(toml::to_string(&config).unwrap().as_str())
        .unwrap();

    let mut cmd = assert_cmd::Command::cargo_bin(BIN_NAME).unwrap();
    cmd.current_dir(test_root.path())
        .args(["--env-path", "config.toml"])
        .assert()
        .success();

    test_root
        .child("stdout")
        .assert(predicate::str::diff("hello, world!\n"));
    test_root
        .child("output")
        .assert(predicate::str::starts_with("Exited Normally"));
}
