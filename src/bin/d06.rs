use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Entry {
  target: String,
  sattelite: String,
}

// returns (source, sattelite)
fn split(s: &str) -> Entry {
  let mut vals = s.split(")");
  Entry {
    target: vals.next().unwrap().to_owned(),
    sattelite: vals.next().unwrap().to_owned(),
  }
}

fn parse_input() -> Vec<Entry> {
  let contents = fs::read_to_string("assets/day6_input").unwrap();
  contents.trim().lines().map(|l| split(l)).collect()
}

fn orbit_count(input: Vec<Entry>) -> usize {
  // First we build a map of all orbits. Key is the sattelite
  // and value is the target.
  let mut map = HashMap::new();
  for v in input.iter() {
    map.insert(v.sattelite.clone(), v.target.clone());
  }

  let mut count = 0;

  let com = "COM".to_owned();

  for k in map.keys() {
    let mut t = k.clone();
    while t != com {
      t = match map.get(&t) {
        Some(v) => {
          count += 1;
          v.clone()
        }
        None => break,
      }
    }
  }

  count
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_simple_orbits() {
    let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
    let data = input.trim().lines().map(|l| split(l)).collect();
    assert_eq!(42, orbit_count(data));
  }
}

fn main() {
  println!("Orbit count: {}", orbit_count(parse_input()));
}
