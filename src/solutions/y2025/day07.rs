pub fn count_beam_splits(input: &str) -> String {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .filter(|row: &Vec<char>| !row.is_empty())
        .collect();

    if grid.is_empty() {
        return "0".to_string();
    }

    // Find starting position (S)
    let mut start_col = 0;
    for row in &grid {
        for (c, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                start_col = c;
                break;
            }
        }
    }

    let cols = grid[0].len();
    let mut split_count = 0u64;

    // Track active beam positions (column indices)
    let mut active_beams: std::collections::HashSet<usize> = std::collections::HashSet::new();
    active_beams.insert(start_col);

    // Process row by row
    for row_data in grid.iter().skip(1) {
        let mut next_beams: std::collections::HashSet<usize> = std::collections::HashSet::new();

        for &beam_col in &active_beams {
            if beam_col >= cols {
                continue;
            }

            let cell = row_data[beam_col];
            match cell {
                '.' => {
                    // Beam continues downward
                    next_beams.insert(beam_col);
                }
                '^' => {
                    // Beam is split: remove current beam, add beams at left and right
                    split_count += 1;
                    if beam_col > 0 {
                        next_beams.insert(beam_col - 1);
                    }
                    if beam_col + 1 < cols {
                        next_beams.insert(beam_col + 1);
                    }
                }
                _ => {
                    // Unknown cell, beam stops
                }
            }
        }

        active_beams = next_beams;
    }

    split_count.to_string()
}

pub fn count_beam_splits_part2(input: &str) -> String {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .filter(|row: &Vec<char>| !row.is_empty())
        .collect();

    if grid.is_empty() {
        return "0".to_string();
    }

    // Find starting position (S)
    let mut start_col = 0;
    for row in &grid {
        for (c, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                start_col = c;
                break;
            }
        }
    }

    let rows = grid.len();
    let cols = grid[0].len();

    // Use DP: track number of ways (timelines) to reach each position
    // dp[row][col] = number of timelines that reach position (row, col)
    let mut dp: Vec<Vec<u64>> = vec![vec![0; cols]; rows];
    dp[0][start_col] = 1;

    // Process row by row
    for (row_idx, row_data) in grid.iter().enumerate().skip(1) {
        for col in 0..cols {
            if dp[row_idx - 1][col] == 0 {
                continue;
            }

            let cell = row_data[col];
            match cell {
                '.' => {
                    // Particle continues downward: one timeline continues
                    dp[row_idx][col] += dp[row_idx - 1][col];
                }
                '^' => {
                    // Particle splits: creates two timelines (left and right)
                    if col > 0 {
                        dp[row_idx][col - 1] += dp[row_idx - 1][col];
                    }
                    if col + 1 < cols {
                        dp[row_idx][col + 1] += dp[row_idx - 1][col];
                    }
                }
                _ => {
                    // Unknown cell, particle stops
                }
            }
        }
    }

    // Count all timelines that reach the final row
    let total_timelines: u64 = dp[rows - 1].iter().sum();
    total_timelines.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(count_beam_splits(input), "21");
    }

    #[test]
    fn test_example_part2() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(count_beam_splits_part2(input), "40");
    }
}
