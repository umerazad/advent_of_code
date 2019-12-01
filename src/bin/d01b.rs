use aoc2019::read_input;

fn fuel(w: i64) -> i64 {
  let v = (w / 3) - 2;
  if v > 0 {
    return v;
  }
  0
}

fn calculate_part_fuel(w: i64) -> i64 {
  let mut result = 0;
  let mut tmp = fuel(w);
  while tmp > 0 {
    result += tmp;
    tmp = fuel(tmp);
  }
  result
}

fn calculate_fuel(weights: Vec<i64>) -> i64 {
  weights
    .iter()
    .fold(0, |acc, &n| acc + calculate_part_fuel(n))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_calculate_fuel() {
    let input = vec![14, 1969, 100756];
    assert_eq!(calculate_fuel(input), 2 + 966 + 50346);
  }
}

fn main() {
  println!(
    "Fuel needed: {}",
    calculate_fuel(read_input("assets/day1_input"))
  );
}
