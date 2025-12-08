pub fn count_fresh_ingredients(input: &str) -> String {
    let parts: Vec<&str> = input.split("\n\n").collect();
    if parts.len() < 2 {
        return "0".to_string();
    }

    // Parse ranges
    let ranges: Vec<(u64, u64)> = parts[0]
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            let parts: Vec<&str> = line.split('-').collect();
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

    // Parse available ingredient IDs
    let available_ids: Vec<u64> = parts[1]
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            line.parse::<u64>().ok()
        })
        .collect();

    // Count how many available IDs are fresh (fall into any range)
    let mut count = 0;
    for &id in &available_ids {
        let is_fresh = ranges.iter().any(|&(start, end)| id >= start && id <= end);
        if is_fresh {
            count += 1;
        }
    }

    count.to_string()
}

pub fn count_fresh_ingredients_part2(input: &str) -> String {
    let parts: Vec<&str> = input.split("\n\n").collect();
    if parts.is_empty() {
        return "0".to_string();
    }

    // Parse ranges
    let mut ranges: Vec<(u64, u64)> = parts[0]
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            let parts: Vec<&str> = line.split('-').collect();
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

    if ranges.is_empty() {
        return "0".to_string();
    }

    // Sort ranges by start
    ranges.sort_by_key(|&(start, _)| start);

    // Merge overlapping ranges
    let mut merged: Vec<(u64, u64)> = vec![ranges[0]];
    for &(start, end) in &ranges[1..] {
        let last = merged.last_mut().unwrap();
        if start <= last.1 + 1 {
            // Overlapping or adjacent - merge
            last.1 = last.1.max(end);
        } else {
            // Non-overlapping - add new range
            merged.push((start, end));
        }
    }

    // Count all IDs in merged ranges
    let mut count = 0u64;
    for (start, end) in merged {
        count += end - start + 1;
    }

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(count_fresh_ingredients(input), "3");
    }

    #[test]
    fn test_example_part2() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(count_fresh_ingredients_part2(input), "14");
    }
}
