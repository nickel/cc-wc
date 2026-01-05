use ccwc::{count_bytes, count_chars, count_lines, count_words};

#[test]
fn test_count_bytes() {
    assert_eq!(count_bytes("hello"), 5);
    assert_eq!(count_bytes(""), 0);
    assert_eq!(count_bytes("hello world"), 11);
}

#[test]
fn test_count_bytes_unicode() {
    assert_eq!(count_bytes("héllo"), 6); // é is 2 bytes in UTF-8
    assert_eq!(count_bytes("日本語"), 9); // 3 chars × 3 bytes each
}

#[test]
fn test_count_lines() {
    assert_eq!(count_lines("hello"), 1);
    assert_eq!(count_lines("hello\nworld"), 2);
    assert_eq!(count_lines("one\ntwo\nthree"), 3);
    assert_eq!(count_lines(""), 0);
}

#[test]
fn test_count_words() {
    assert_eq!(count_words("hello world"), 2);
    assert_eq!(count_words("  hello   world  "), 2);
    assert_eq!(count_words("one"), 1);
    assert_eq!(count_words(""), 0);
    assert_eq!(count_words("multiple   spaces   between"), 3);
}

#[test]
fn test_count_chars() {
    assert_eq!(count_chars("hello"), 5);
    assert_eq!(count_chars("héllo"), 5); // é is 1 char
    assert_eq!(count_chars("日本語"), 3);
    assert_eq!(count_chars(""), 0);
}
