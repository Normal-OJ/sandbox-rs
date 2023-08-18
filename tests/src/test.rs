use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use toml;

const BIN_NAME: &'static str = "noj_sandbox";

fn common_compile_args() -> &'static [&'static str] {
    &["-DONLINE_JUDGE", "-O2", "-w", "-fmax-errors=3"]
}

fn create_config(test_root: &assert_fs::TempDir, config_path: &str) {
    let mut config = toml::toml! {
        cwd = "./"
        large-stack = true
        max-process = 10
        memory-limit = 5120000
        output-size-limit = 10000
        runtime-limit = 1500
        lang = 1
    };
    for f in ["stdin", "stdout", "stderr", "output"] {
        config.insert(
            f.to_string(),
            toml::Value::String(test_root.path().join(f).to_str().unwrap().to_owned()),
        );
    }
    test_root
        .child(config_path)
        .write_str(toml::to_string(&config).unwrap().as_str())
        .unwrap();
}

/// Compile `{test_root}/main.c` into `{test_root}/main`
fn compile_c(test_root: &assert_fs::TempDir) {
    let mut compile_cmd = assert_cmd::Command::new("gcc");
    compile_cmd
        .args(common_compile_args())
        .args(["-std=c++11", "main.c", "-o", "main", "-lm"])
        .current_dir(test_root.path())
        .assert()
        .success();
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
    compile_c(&test_root);

    test_root.child("stdin").touch().unwrap();
    let config_path = "config.toml";
    create_config(&test_root, config_path);

    let mut cmd = assert_cmd::Command::cargo_bin(BIN_NAME).unwrap();
    cmd.current_dir(test_root.path())
        .args(["--env-path", config_path])
        .assert()
        .success();

    test_root
        .child("stdout")
        .assert(predicate::str::diff("hello, world!\n"));
    test_root
        .child("output")
        .assert(predicate::str::starts_with("Exited Normally"));
}

#[test]
fn test_c_code_catch_runtime_error() {
    let test_root = assert_fs::TempDir::new().unwrap();
    test_root
        .child("main.c")
        .write_str(
            r#"
#include <stdio.h>

int main()
{
    int a = 1;
    int b = 0;
    int c = a / b;
    printf("%d\n", c);
    return 0;
}
"#,
        )
        .unwrap();
    compile_c(&test_root);

    test_root.child("stdin").touch().unwrap();
    let config_path = "config.toml";
    create_config(&test_root, config_path);

    let mut cmd = assert_cmd::Command::cargo_bin(BIN_NAME).unwrap();
    cmd.current_dir(test_root.path())
        .args(["--env-path", config_path])
        .assert()
        .success();

    test_root.child("stdout").assert(predicate::str::is_empty());
    test_root
        .child("output")
        .assert(predicate::str::starts_with("RE"));
}
