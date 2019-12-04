use std::collections::HashMap;

fn count_valid_passwords() -> i32 {
  let mut count = 0;
  for n in 264793..803936 {
    let digits: Vec<u32> = n
      .to_string()
      .chars()
      .map(|c| c.to_digit(10).unwrap())
      .collect();

    if consecutive_duplicates(&digits)
      && non_decreasing(&digits)
      && atleast_one_digit_twice(&digits)
    {
      count += 1;
    }
  }

  count
}

// part 2 condition
fn atleast_one_digit_twice(digits: &Vec<u32>) -> bool {
  let mut m = HashMap::new();
  for n in digits {
    let count = m.entry(n).or_insert(0);
    *count += 1;
  }

  for &v in m.values() {
    if v == 2 {
      return true;
    }
  }

  false
}

fn consecutive_duplicates(digits: &Vec<u32>) -> bool {
  for i in 0..digits.len() - 1 {
    if digits[i] == digits[i + 1] {
      return true;
    }
  }

  false
}

fn non_decreasing(digits: &Vec<u32>) -> bool {
  for i in 0..digits.len() - 1 {
    if digits[i] > digits[i + 1] {
      return false;
    }
  }

  true
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_count_valid_passwords() {
    assert_eq!(count_valid_passwords(), 628);
  }
}

fn main() {
  let count = count_valid_passwords();
  println!("Count: {}", count);
}
