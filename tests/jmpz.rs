mod common;

#[test]
fn test_jmpz_valid() {
    let result = common::run_test_output("tests/input/jmpz.test");

    assert_eq!(result, "Goodbye!\n");
}

#[test]
fn test_jmpz_invalid() {
    let result = common::run_test_status("tests/input/jmpz.test");

    assert_eq!(result.success(), true);
}
