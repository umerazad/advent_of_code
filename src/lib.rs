use std::fs;

pub fn read_input(path: &str) -> Vec<i64> {
    let contents = fs::read_to_string(path).unwrap();
    contents
        .lines()
        .map(|l| l.trim().parse::<i64>().unwrap())
        .collect()
}

pub fn read_csv_ints(path: &str) -> Vec<usize> {
    let contents = fs::read_to_string(path).unwrap();
    contents
        .split(",")
        .filter_map(|v| v.parse::<usize>().ok())
        .collect()
}

pub fn read_csv_strings(path: &str) -> Vec<String> {
    let contents = fs::read_to_string(path).unwrap();
    contents
        .split(",")
        .map(|l| l.trim().to_owned())
        .filter(|l| l.len() > 1)
        .collect()
}
