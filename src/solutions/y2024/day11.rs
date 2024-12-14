use std::collections::HashMap;

/// Simulates the evolution of stones for a given number of blinks.
pub fn count_stones_after_blinks(input: &str, blinks: usize) -> String {
    let stones: Vec<u64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let total_stones = simulate_stones_evolution(stones, blinks);
    total_stones.to_string()
}

/// Simulates the evolution of stones for Part 1 or Part 2.
fn simulate_stones_evolution(stones: Vec<u64>, blinks: usize) -> usize {
    let mut stone_counts: HashMap<u64, usize> = HashMap::new();
    for &stone in &stones {
        *stone_counts.entry(stone).or_insert(0) += 1;
    }

    for _ in 0..blinks {
        let mut next_counts = HashMap::new();

        for (&stone, &count) in &stone_counts {
            if stone == 0 {
                // Rule 1: Replace 0 with 1
                *next_counts.entry(1).or_insert(0) += count;
            } else if stone >= 10 && stone.to_string().len() % 2 == 0 {
                // Rule 2: Split into two stones
                let digits = stone.to_string();
                let mid = digits.len() / 2;
                let left: u64 = digits[..mid].parse().unwrap_or(0);
                let right: u64 = digits[mid..].parse().unwrap_or(0);
                *next_counts.entry(left).or_insert(0) += count;
                *next_counts.entry(right).or_insert(0) += count;
            } else {
                // Rule 3: Multiply by 2024
                *next_counts.entry(stone * 2024).or_insert(0) += count;
            }
        }

        stone_counts = next_counts;
    }

    stone_counts.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_stones_after_blinks_part1() {
        let input = "0 1 10 99 999";
        assert_eq!(count_stones_after_blinks(input, 25), "125681");
    }

    #[test]
    fn test_count_stones_after_blinks_part2() {
        let input = "0 1 10 99 999";
        assert_eq!(count_stones_after_blinks(input, 75), "149161030616311");
    }

    #[test]
    fn test_small_example() {
        let input = "125 17";
        assert_eq!(count_stones_after_blinks(input, 1), "3");
        assert_eq!(count_stones_after_blinks(input, 2), "4");
    }
}
