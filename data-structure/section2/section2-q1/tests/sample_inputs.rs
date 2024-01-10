use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"6 3
3 1 4 1 5 9
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1\n4\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"9 0
1 2 3 4 5 6 7 8 9
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1\n9\n");
    assert!(output.stderr_str().is_empty());
}
