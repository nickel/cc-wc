pub fn count_bytes(content: &str) -> usize {
    content.len()
}

pub fn count_lines(content: &str) -> usize {
    content.lines().count()
}

pub fn count_words(content: &str) -> usize {
    content.split_whitespace().count()
}

pub fn count_chars(content: &str) -> usize {
    content.chars().count()
}
