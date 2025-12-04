pub fn sum_max_joltage(input: &str) -> String {
    let mut total = 0u64;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Find the maximum 2-digit number we can form by selecting exactly 2 digits
        let max_joltage = find_max_two_digit_number(line);
        total += max_joltage;
    }

    total.to_string()
}

pub fn sum_max_joltage_part2(input: &str) -> String {
    let mut total = 0u64;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Find the maximum 12-digit number we can form by selecting exactly 12 digits
        let max_joltage = find_max_twelve_digit_number_v2(line);
        total += max_joltage;
    }

    total.to_string()
}

fn find_max_two_digit_number(line: &str) -> u64 {
    let digits: Vec<u8> = line
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u8))
        .collect();

    if digits.len() < 2 {
        return 0;
    }

    let mut max_value = 0u64;

    // Try all pairs of digits (i < j to maintain order)
    for i in 0..digits.len() {
        for j in (i + 1)..digits.len() {
            let two_digit = (digits[i] as u64) * 10 + (digits[j] as u64);
            max_value = max_value.max(two_digit);
        }
    }

    max_value
}

// Select 12 digits greedily to maximize the number
fn find_max_twelve_digit_number_v2(line: &str) -> u64 {
    let digits: Vec<u8> = line
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u8))
        .collect();

    if digits.len() < 12 {
        return 0;
    }

    // We need to select exactly 12 digits
    // Use greedy selection: at each of the 12 positions, select the best possible digit
    // that still allows us to complete the selection

    let n = digits.len();
    let k = 12;
    let mut selected_indices = Vec::new();

    // Greedy: for each of the 12 positions, select the best possible digit
    for pos in 0..k {
        // We need to select (k - pos) more digits after this one
        // We can choose from digits starting at some index
        let start_idx = if selected_indices.is_empty() {
            0
        } else {
            selected_indices.last().unwrap() + 1
        };
        let remaining_digits = n - start_idx;
        let needed_digits = k - pos;

        // We can look ahead, but must leave enough digits for the remaining selections
        let max_look_ahead = remaining_digits - needed_digits;
        let end_idx = start_idx + max_look_ahead + 1;

        // Find the best digit we can select in this range
        let mut best_idx = start_idx;
        let mut best_digit = digits[start_idx];

        for (i, &digit) in digits
            .iter()
            .enumerate()
            .skip(start_idx)
            .take(end_idx.min(n) - start_idx)
        {
            if digit > best_digit {
                best_digit = digit;
                best_idx = i;
            }
        }

        selected_indices.push(best_idx);
    }

    // Convert to number
    let mut result = 0u64;
    for &idx in &selected_indices {
        result = result * 10 + digits[idx] as u64;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_two_digit() {
        assert_eq!(find_max_two_digit_number("987654321111111"), 98);
        assert_eq!(find_max_two_digit_number("811111111111119"), 89);
        assert_eq!(find_max_two_digit_number("234234234234278"), 78);
        assert_eq!(find_max_two_digit_number("818181911112111"), 92);
    }

    #[test]
    fn test_example() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(sum_max_joltage(input), "357");
    }

    #[test]
    fn test_find_max_twelve_digit() {
        assert_eq!(
            find_max_twelve_digit_number_v2("987654321111111"),
            987654321111
        );
        assert_eq!(
            find_max_twelve_digit_number_v2("811111111111119"),
            811111111119
        );
        assert_eq!(
            find_max_twelve_digit_number_v2("234234234234278"),
            434234234278
        );
        assert_eq!(
            find_max_twelve_digit_number_v2("818181911112111"),
            888911112111
        );
    }

    #[test]
    fn test_example_part2() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(sum_max_joltage_part2(input), "3121910778619");
    }
}
