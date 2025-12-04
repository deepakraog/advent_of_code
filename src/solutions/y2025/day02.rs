pub fn sum_invalid_ids(input: &str) -> String {
    let mut total = 0u64;

    // Parse ranges from input (comma-separated)
    let ranges: Vec<(u64, u64)> = input
        .trim()
        .split(',')
        .filter_map(|range_str| {
            let parts: Vec<&str> = range_str.split('-').collect();
            if parts.len() == 2 {
                if let (Ok(start), Ok(end)) = (parts[0].parse::<u64>(), parts[1].parse::<u64>()) {
                    Some((start, end))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    // Check each range
    for (start, end) in ranges {
        for id in start..=end {
            if is_invalid_id(id) {
                total += id;
            }
        }
    }

    total.to_string()
}

pub fn sum_invalid_ids_part2(input: &str) -> String {
    let mut total = 0u64;

    // Parse ranges from input (comma-separated)
    let ranges: Vec<(u64, u64)> = input
        .trim()
        .split(',')
        .filter_map(|range_str| {
            let parts: Vec<&str> = range_str.split('-').collect();
            if parts.len() == 2 {
                if let (Ok(start), Ok(end)) = (parts[0].parse::<u64>(), parts[1].parse::<u64>()) {
                    Some((start, end))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    // Check each range
    for (start, end) in ranges {
        for id in start..=end {
            if is_invalid_id_part2(id) {
                total += id;
            }
        }
    }

    total.to_string()
}

fn is_invalid_id(id: u64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();

    // An invalid ID must have even length (since it's a sequence repeated twice)
    if !len.is_multiple_of(2) {
        return false;
    }

    let half_len = len / 2;
    let first_half = &id_str[..half_len];
    let second_half = &id_str[half_len..];

    // Check if the second half equals the first half
    first_half == second_half
}

fn is_invalid_id_part2(id: u64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();

    // An invalid ID must be made of a sequence repeated at least twice
    // Try all possible sequence lengths (divisors of len where we have at least 2 repetitions)
    for seq_len in 1..=len / 2 {
        if !len.is_multiple_of(seq_len) {
            continue; // Can't divide evenly
        }

        let num_repetitions = len / seq_len;
        if num_repetitions < 2 {
            continue; // Need at least 2 repetitions
        }

        // Get the first sequence
        let first_seq = &id_str[..seq_len];

        // Check if all repetitions match
        let mut all_match = true;
        for i in 1..num_repetitions {
            let start = i * seq_len;
            let end = start + seq_len;
            if &id_str[start..end] != first_seq {
                all_match = false;
                break;
            }
        }

        if all_match {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_id() {
        assert!(is_invalid_id(11));
        assert!(is_invalid_id(6464));
        assert!(is_invalid_id(123123));
        assert!(!is_invalid_id(101));
        assert!(!is_invalid_id(123));
        assert!(!is_invalid_id(1234));
    }

    #[test]
    fn test_example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(sum_invalid_ids(input), "1227775554");
    }

    #[test]
    fn test_invalid_id_part2() {
        assert!(is_invalid_id_part2(12341234)); // 1234 repeated 2 times
        assert!(is_invalid_id_part2(123123123)); // 123 repeated 3 times
        assert!(is_invalid_id_part2(1212121212)); // 12 repeated 5 times
        assert!(is_invalid_id_part2(1111111)); // 1 repeated 7 times
        assert!(is_invalid_id_part2(11)); // Still valid (1 repeated 2 times)
        assert!(!is_invalid_id_part2(123)); // Not a repetition
    }

    #[test]
    fn test_example_part2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(sum_invalid_ids_part2(input), "4174379265");
    }
}
