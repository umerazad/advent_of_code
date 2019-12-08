use std::fs;

fn calculate_layer(layer: &[u32]) -> (u64, u64) {
  let mut zero_count = 0;
  let mut one_count = 0;
  let mut two_count = 0;

  for &i in layer {
    match i {
      0 => zero_count += 1,
      1 => one_count += 1,
      2 => two_count += 1,
      _ => (),
    }
  }

  (zero_count, one_count * two_count)
}

fn main() {
  let contents = fs::read_to_string("assets/day8_input").unwrap();
  let digits: Vec<u32> = contents
    .trim()
    .chars()
    .map(|c| c.to_digit(10).unwrap())
    .collect();

  let mut min = u64::max_value();
  let mut result = 0;
  let chunks = (&digits[..]).chunks(25 * 6);
  for c in chunks {
    let counts = calculate_layer(c);
    if counts.0 < min {
      result = counts.1;
      min = counts.0;
    }
  }

  println!("Result: {}", result);
}
