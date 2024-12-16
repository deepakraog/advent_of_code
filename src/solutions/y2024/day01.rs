use std::collections::HashMap;

pub fn calculate_total_distance(input: &str) -> String {
    // Parse the input into two lists
    let (mut left_list, mut right_list) = parse_input(input);

    // Sort the lists
    left_list.sort();
    right_list.sort();

    // Calculate the total distance
    let total_distance: i32 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    total_distance.to_string()
}

pub fn calculate_similarity_score(input: &str) -> String {
    // Parse the input into two lists
    let (left_list, right_list) = parse_input(input);

    // Count occurrences of each number in the right list
    let mut right_count = HashMap::new();
    for &num in &right_list {
        *right_count.entry(num).or_insert(0) += 1;
    }

    // Calculate similarity score
    let similarity_score: i32 = left_list
        .iter()
        .map(|&num| num * right_count.get(&num).unwrap_or(&0))
        .sum();

    similarity_score.to_string()
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        if numbers.len() == 2 {
            left_list.push(numbers[0]);
            right_list.push(numbers[1]);
        }
    }

    (left_list, right_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_distance() {
        let input = "1 3\n2 4\n5 8";
        assert_eq!(calculate_total_distance(input), "7");
    }

    #[test]
    fn test_similarity_score() {
        let input = "1 1\n2 2\n3 3";
        assert_eq!(calculate_similarity_score(input), "6");
    }
}
