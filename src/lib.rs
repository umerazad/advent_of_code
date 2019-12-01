use std::fs;

pub fn read_input(path: &str) -> Vec<i64> {
    let contents = fs::read_to_string(path).unwrap();
    contents
        .lines()
        .map(|l| l.trim().parse::<i64>().unwrap())
        .collect()
}
