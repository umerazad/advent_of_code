use aoc2019::read_input;

fn calculate_fuel(weights: Vec<i64>) -> i64 {
    weights.iter().fold(0, |acc, n| acc + (n / 3) - 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_fuel() {
        let input = vec![12, 14, 1969, 100756];
        assert_eq!(calculate_fuel(input), 2 + 2 + 654 + 33583);
    }
}

fn main() {
    println!(
        "Fuel needed: {}",
        calculate_fuel(read_input("assets/day1_input"))
    );
}
