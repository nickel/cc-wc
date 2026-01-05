use std::process::Command;

fn run_ccwc(args: &[&str]) -> String {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--"])
        .args(args)
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

#[test]
fn test_bytes_flag() {
    let output = run_ccwc(&["-c", "tests/fixtures/text.txt"]);
    // Note: our implementation counts UTF-8 string bytes, not raw file bytes
    assert_eq!(output, "342190 tests/fixtures/text.txt");
}

#[test]
fn test_lines_flag() {
    let output = run_ccwc(&["-l", "tests/fixtures/text.txt"]);
    assert_eq!(output, "7145 tests/fixtures/text.txt");
}

#[test]
fn test_words_flag() {
    let output = run_ccwc(&["-w", "tests/fixtures/text.txt"]);
    assert_eq!(output, "58164 tests/fixtures/text.txt");
}

#[test]
fn test_chars_flag() {
    let output = run_ccwc(&["-m", "tests/fixtures/text.txt"]);
    assert_eq!(output, "339292 tests/fixtures/text.txt");
}

#[test]
fn test_combined_flags_lw() {
    let output = run_ccwc(&["-lw", "tests/fixtures/text.txt"]);
    assert_eq!(output, "7145 58164 tests/fixtures/text.txt");
}

#[test]
fn test_combined_flags_lwc() {
    let output = run_ccwc(&["-lwc", "tests/fixtures/text.txt"]);
    assert_eq!(output, "7145 58164 342190 tests/fixtures/text.txt");
}

#[test]
fn test_separate_flags() {
    let output = run_ccwc(&["-l", "-w", "-c", "tests/fixtures/text.txt"]);
    assert_eq!(output, "7145 58164 342190 tests/fixtures/text.txt");
}
