use std::collections::VecDeque;

/// Generates the pseudorandom sequence of secret numbers and returns the 2000th secret number.
fn generate_secret_numbers(mut secret: u32, count: usize) -> Vec<u32> {
    let mut results = Vec::new();

    for _ in 0..count {
        // Step 1: Multiply by 64 and mix
        secret = secret.wrapping_mul(64) ^ secret;
        secret %= 16777216;

        // Step 2: Divide by 32, round down, and mix
        let temp = secret / 32;
        secret ^= temp;
        secret %= 16777216;

        // Step 3: Multiply by 2048 and mix
        secret = secret.wrapping_mul(2048) ^ secret;
        secret %= 16777216;

        results.push(secret);
    }

    results
}

/// Part 1: Simulate the 2000th secret number for each buyer and compute their sum.
pub fn solve_part1(input: &str) -> String {
    let secrets: Vec<u32> = input
        .lines()
        .filter_map(|line| line.trim().parse::<u32>().ok())
        .collect();

    let mut total = 0;
    for &secret in &secrets {
        let results = generate_secret_numbers(secret, 2000);
        total += *results.last().unwrap_or(&0) as u64; // Use u64 to handle large sums
    }

    total.to_string()
}

/// Calculates price changes for a sequence of prices.
fn calculate_price_changes(prices: &[u32]) -> Vec<i8> {
    prices
        .windows(2)
        .map(|window| (window[1] % 10) as i8 - (window[0] % 10) as i8)
        .collect()
}

/// Part 2: Find the best sequence of four price changes that maximizes the bananas obtained.
pub fn solve_part2(input: &str) -> String {
    let secrets: Vec<u32> = input
        .lines()
        .filter_map(|line| line.trim().parse::<u32>().ok())
        .collect();

    let mut max_bananas = 0;
    let mut best_sequence = vec![0; 4];

    // Generate all possible sequences of four price changes
    let range = -9..=9; // Price changes range from -9 to 9
    for a in range.clone() {
        for b in range.clone() {
            for c in range.clone() {
                for d in range.clone() {
                    let sequence = vec![a, b, c, d];
                    let mut bananas = 0;

                    for &secret in &secrets {
                        let prices = generate_secret_numbers(secret, 2000);
                        let changes = calculate_price_changes(&prices);

                        let mut queue = VecDeque::new();
                        for (i, &change) in changes.iter().enumerate() {
                            queue.push_back(change);
                            if queue.len() > 4 {
                                queue.pop_front();
                            }

                            if queue.len() == 4
                                && queue.iter().copied().eq(sequence.iter().copied())
                            {
                                bananas += (prices[i + 1] % 10) as usize; // Add the price when the sequence occurs
                                break; // Only sell once per buyer
                            }
                        }
                    }

                    if bananas > max_bananas {
                        max_bananas = bananas;
                        best_sequence = sequence;
                    }
                }
            }
        }
    }

    format!(
        "Best sequence: {:?}, Max Bananas: {}",
        best_sequence, max_bananas
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"1
10
100
2024";

        assert_eq!(solve_part1(input), "37327623");
    }
}
