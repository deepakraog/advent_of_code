use std::collections::VecDeque;

pub fn calculate_checksum(input: &str) -> String {
    solve(input, false).to_string()
}

pub fn calculate_checksum_fragmentation(input: &str) -> String {
    solve(input, true).to_string()
}

fn solve(input: &str, part2: bool) -> i64 {
    let mut files = VecDeque::new();
    let mut spaces = VecDeque::new();
    let mut final_array = Vec::new();
    let mut file_id = 0;
    let mut position = 0;

    // Parse the input into files and spaces
    for (i, c) in input.chars().enumerate() {
        if let Some(count) = c.to_digit(10) {
            let count = count as usize;
            if i % 2 == 0 {
                // File blocks
                if part2 {
                    files.push_back((position, count, file_id));
                }
                for _ in 0..count {
                    final_array.push(Some(file_id));
                    if !part2 {
                        files.push_back((position, 1, file_id));
                    }
                    position += 1;
                }
                file_id += 1;
            } else {
                // Space blocks
                spaces.push_back((position, count));
                for _ in 0..count {
                    final_array.push(None);
                    position += 1;
                }
            }
        } else {
            // Ignore invalid characters or input
            continue;
        }
    }

    // Compact the array
    while let Some((file_pos, file_size, file_id)) = files.pop_back() {
        let mut moved = false;
        for (space_pos, space_size) in spaces.iter_mut() {
            if *space_pos < file_pos && file_size <= *space_size {
                // Move file into the space
                for i in 0..file_size {
                    assert_eq!(final_array[file_pos + i], Some(file_id));
                    final_array[file_pos + i] = None;
                    final_array[*space_pos + i] = Some(file_id);
                }
                // Update the space
                *space_pos += file_size;
                *space_size -= file_size;
                moved = true;
                break;
            }
        }

        if !moved {
            continue; // Skip files that cannot be moved
        }
    }

    // Calculate the checksum
    final_array
        .iter()
        .enumerate()
        .filter_map(|(i, &block)| block.map(|id| i as i64 * id as i64))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_checksum() {
        let input = "2333133121414131402";
        assert_eq!(calculate_checksum(input), "1928");
    }

    #[test]
    fn test_calculate_checksum_fragmentation() {
        let input = "2333133121414131402";
        assert_eq!(calculate_checksum_fragmentation(input), "2858");
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        assert_eq!(calculate_checksum(input), "0");
        assert_eq!(calculate_checksum_fragmentation(input), "0");
    }
}
