use std::process::Command;

#[test]
fn test_jmpz_valid() {
    let filename = "tests/jmpz.test";
    let output = Command::new("cargo")
        .arg("run")
        .arg(filename)
        .output()
        .expect("failed to run jmpz test");

    let result = String::from_utf8_lossy(&output.stdout);

    assert_eq!(result, "Goodbye!\n");
}

#[test]
fn test_jmpz_invalid() {
    let filename = "tests/jmpz_invalid.test";
    let status = Command::new("cargo")
        .arg("run")
        .arg(filename)
        .status()
        .expect("failed to run jmpz test");

    assert_eq!(status.success(), false);
}
