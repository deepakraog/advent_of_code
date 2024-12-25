use std::collections::{HashMap, HashSet};

/// Computes the next secret value in the sequence.
const fn next_secret(secret: i64) -> i64 {
    let secret = (secret ^ (secret * 64)) % 16_777_216;
    let secret = (secret ^ (secret / 32)) % 16_777_216;
    (secret ^ (secret * 2048)) % 16_777_216
}

/// Represents the puzzle with its initial secrets.
pub struct Puzzle {
    initial_secrets: Vec<i64>,
}

impl Puzzle {
    /// Creates a new Puzzle instance.
    pub fn new() -> Self {
        Self {
            initial_secrets: Vec::new(),
        }
    }

    /// Configures the puzzle with input data.
    pub fn configure(&mut self, input: &str) {
        self.initial_secrets
            .extend(input.lines().map_while(|s| s.parse::<i64>().ok()));
    }

    /// Solves part one of the puzzle.
    pub fn solve_part1(&self) -> i64 {
        self.initial_secrets
            .iter()
            .map(|&initial_secret| (0..2000).fold(initial_secret, |secret, _| next_secret(secret)))
            .sum()
    }

    /// Solves part two of the puzzle.
    pub fn solve_part2(&self) -> i64 {
        let mut bananas = HashMap::new();

        for &initial_secret in &self.initial_secrets {
            let mut prices = Vec::new();

            let mut secret = initial_secret;
            prices.push(secret % 10);
            for _ in 0..2000 {
                secret = next_secret(secret);
                prices.push(secret % 10);
            }

            let mut seen = HashSet::new();
            for p in prices.windows(5) {
                let sequence = [p[1] - p[0], p[2] - p[1], p[3] - p[2], p[4] - p[3]];

                if seen.insert(sequence) {
                    *bananas.entry(sequence).or_default() += p[4];
                }
            }
        }

        *bananas.values().max().unwrap()
    }
}

impl Default for Puzzle {
    fn default() -> Self {
        Self::new()
    }
}

pub fn solve_part1(input: &str) -> String {
    let mut puzzle = Puzzle::new();
    puzzle.configure(input);
    puzzle.solve_part1().to_string()
}

pub fn solve_part2(input: &str) -> String {
    let mut puzzle = Puzzle::new();
    puzzle.configure(input);
    puzzle.solve_part2().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "123456\n654321\n";
        assert_eq!(solve_part1(input), "12178264");
    }

    #[test]
    fn test_part2() {
        let input = "123456\n654321\n";
        assert_eq!(solve_part2(input), "18");
    }
}
