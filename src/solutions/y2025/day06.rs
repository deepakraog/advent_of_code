pub fn calculate_grand_total(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return "0".to_string();
    }

    // Find the operation line (last line with + or *)
    let mut op_line_idx = 0;
    for (i, line) in lines.iter().enumerate().rev() {
        if line.contains('+') || line.contains('*') {
            op_line_idx = i;
            break;
        }
    }

    let op_line = lines[op_line_idx];
    let num_lines = &lines[..op_line_idx];

    // Find all columns (positions where there's a + or *)
    // Numbers are right-aligned, so the operation is at the right edge
    let mut columns: Vec<(usize, char)> = Vec::new();
    for (i, ch) in op_line.char_indices() {
        if ch == '+' || ch == '*' {
            columns.push((i, ch));
        }
    }

    let mut grand_total = 0u64;

    // Process each column
    for (col_pos, op) in columns {
        let mut numbers = Vec::new();

        // Extract numbers from this column position
        // Numbers are right-aligned, so we look left from the operation position
        for line in num_lines {
            let line_chars: Vec<char> = line.chars().collect();
            if col_pos < line_chars.len() {
                // Check if there's a digit at or near this position
                // Look for digits going left from col_pos
                let mut pos = col_pos;

                // Skip spaces to find the rightmost digit
                while pos < line_chars.len() && line_chars[pos] == ' ' {
                    pos += 1;
                }

                // If we found a digit, extract the full number
                if pos < line_chars.len() && line_chars[pos].is_ascii_digit() {
                    // Find the end of the number (rightmost digit)
                    let mut num_end = pos + 1;
                    while num_end < line_chars.len() && line_chars[num_end].is_ascii_digit() {
                        num_end += 1;
                    }

                    // Find the start of the number (leftmost digit)
                    let mut num_start = pos;
                    while num_start > 0 && line_chars[num_start - 1].is_ascii_digit() {
                        num_start -= 1;
                    }

                    if num_start < num_end {
                        let num_str: String = line_chars[num_start..num_end].iter().collect();
                        if let Ok(num) = num_str.parse::<u64>() {
                            numbers.push(num);
                        }
                    }
                }
            }
        }

        if !numbers.is_empty() {
            let result: u64 = if op == '+' {
                numbers.iter().sum()
            } else {
                numbers.iter().product()
            };
            grand_total += result;
        }
    }

    grand_total.to_string()
}

pub fn calculate_grand_total_part2(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return "0".to_string();
    }

    // Find the operation line (last line with + or *)
    let mut op_line_idx = 0;
    for (i, line) in lines.iter().enumerate().rev() {
        if line.contains('+') || line.contains('*') {
            op_line_idx = i;
            break;
        }
    }

    let op_line = lines[op_line_idx];
    let num_lines = &lines[..op_line_idx];

    // Find problem ranges by identifying space columns (separators)
    let max_col = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let mut problem_ranges: Vec<(usize, usize)> = Vec::new();
    let mut current_start: Option<usize> = None;

    for col in 0..=max_col {
        // Check if this column is all spaces (across all lines)
        let mut is_space = true;
        for line in &lines {
            let line_chars: Vec<char> = line.chars().collect();
            if col < line_chars.len() && line_chars[col] != ' ' {
                is_space = false;
                break;
            }
        }

        if is_space {
            if let Some(start) = current_start {
                problem_ranges.push((start, col.saturating_sub(1)));
                current_start = None;
            }
        } else if current_start.is_none() {
            current_start = Some(col);
        }
    }
    if let Some(start) = current_start {
        problem_ranges.push((start, max_col.saturating_sub(1)));
    }

    let mut grand_total = 0u64;

    // Process each problem range (reading right-to-left)
    for (start_col, end_col) in problem_ranges {
        let mut numbers = Vec::new();
        let mut op = ' ';

        // Read columns right-to-left
        for col in (start_col..=end_col).rev() {
            // Extract number from this column (digits top to bottom)
            let mut digits = Vec::new();
            for line in num_lines {
                let line_chars: Vec<char> = line.chars().collect();
                if col < line_chars.len() {
                    let ch = line_chars[col];
                    if ch.is_ascii_digit() {
                        digits.push(ch);
                    }
                }
            }

            if !digits.is_empty() {
                let num_str: String = digits.iter().collect();
                if let Ok(num) = num_str.parse::<u64>() {
                    numbers.push(num);
                }
            }

            // Get operation from this column (if present)
            let op_line_chars: Vec<char> = op_line.chars().collect();
            if col < op_line_chars.len() {
                let ch = op_line_chars[col];
                if ch == '+' || ch == '*' {
                    op = ch;
                }
            }
        }

        if !numbers.is_empty() && (op == '+' || op == '*') {
            let result: u64 = if op == '+' {
                numbers.iter().sum()
            } else {
                numbers.iter().product()
            };
            grand_total += result;
        }
    }

    grand_total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
        assert_eq!(calculate_grand_total(input), "4277556");
    }

    #[test]
    fn test_example_part2() {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
        assert_eq!(calculate_grand_total_part2(input), "3263827");
    }
}
