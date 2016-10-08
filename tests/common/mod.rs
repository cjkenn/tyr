use std::process::{Command, ExitStatus};

pub fn run_test_output(filename: &str) -> String {
    let output = Command::new("cargo")
        .arg("run")
        .arg(filename)
        .output()
        .expect("failed to run test:");

    String::from_utf8(output.stdout).ok().unwrap()
}

pub fn run_test_status(filename: &str) -> ExitStatus {
    Command::new("cargo")
        .arg("run")
        .arg(filename)
        .status()
        .expect("failed to run test:")
}
