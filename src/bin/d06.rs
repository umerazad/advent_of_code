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

fn build_map(input: Vec<Entry>) -> HashMap<String, String> {
    //  Key is the sattelite and value is the target.
    let mut map = HashMap::new();
    for v in input.iter() {
        map.insert(v.sattelite.clone(), v.target.clone());
    }

    map
}

fn find_path(map: &HashMap<String, String>, source: &str, dest: &str) -> Vec<String> {
    let mut path = vec![];

    let mut key = source;
    while key != dest {
        key = match map.get(key) {
            Some(v) => {
                path.push(v.to_owned());
                v
            }
            None => break,
        }
    }
    // remove dest
    path.pop();
    path.reverse();
    path
}

// part 2
fn minimal_orbital_transfers(map: &HashMap<String, String>, source: &str, dest: &str) -> usize {
    let p1 = find_path(map, source, "COM");
    let p2 = find_path(map, dest, "COM");

    // We'll drop the common prefix and rest will be the path.
    let mut i = 0;
    let mut j = 0;
    while i < p1.len() && j < p2.len() {
        if p1[i] != p2[j] {
            break;
        }

        i += 1;
        j += 1;
    }

    p1.len() - i + p2.len() - j
}

// Only needed for part-1.
fn orbit_count(input: Vec<Entry>) -> usize {
    // First we build a map of all orbits. Key is the sattelite
    // and value is the target.
    let map = build_map(input);

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

    fn get_test_input() -> Vec<Entry> {
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
K)L
K)YOU
I)SAN";
        input.trim().lines().map(|l| split(l)).collect()
    }

    #[test]
    fn test_minimal_orbital_transfers() {
        let map = build_map(get_test_input());
        assert_eq!(4, minimal_orbital_transfers(&map, "YOU", "SAN"));
    }

    #[test]
    fn test_find_path() {
        let path = find_path(&build_map(get_test_input()), "L", "COM");
        assert_eq!(6, path.len());
    }

    #[test]
    fn test_simple_orbits() {
        assert_eq!(54, orbit_count(get_test_input()));
    }
}

fn main() {
    let map = build_map(parse_input());
    println!("count: {}", minimal_orbital_transfers(&map, "YOU", "SAN"));
}
