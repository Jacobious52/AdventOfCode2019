pub fn numbers(s: &str) -> Vec<i64> {
    s.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

pub fn numbers_commas(s: &str) -> Vec<i64> {
    s.split(',').map(|s| s.parse().unwrap()).collect()
}

pub fn lines(s: &str) -> Vec<String> {
    s.lines().map(|s| s.to_string()).collect()
}

pub fn words(s: &str) -> Vec<String> {
    s.split_whitespace().map(|s| s.to_string()).collect()
}
