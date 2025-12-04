pub fn count_accessible_rolls(input: &str) -> String {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .filter(|row: &Vec<char>| !row.is_empty())
        .collect();

    if grid.is_empty() {
        return "0".to_string();
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // Directions for 8 adjacent positions
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '@' {
                // Count adjacent rolls of paper
                let mut adjacent_count = 0;
                for (di, dj) in &directions {
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;

                    if ni >= 0
                        && ni < rows as i32
                        && nj >= 0
                        && nj < cols as i32
                        && grid[ni as usize][nj as usize] == '@'
                    {
                        adjacent_count += 1;
                    }
                }

                // Accessible if fewer than 4 adjacent rolls
                if adjacent_count < 4 {
                    count += 1;
                }
            }
        }
    }

    count.to_string()
}

pub fn count_accessible_rolls_part2(input: &str) -> String {
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .filter(|row: &Vec<char>| !row.is_empty())
        .collect();

    if grid.is_empty() {
        return "0".to_string();
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut total_removed = 0;

    // Directions for 8 adjacent positions
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    // Keep removing rolls until no more can be removed
    loop {
        // Find all accessible rolls in current state
        let mut to_remove = Vec::new();

        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == '@' {
                    // Count adjacent rolls of paper
                    let mut adjacent_count = 0;
                    for (di, dj) in &directions {
                        let ni = i as i32 + di;
                        let nj = j as i32 + dj;

                        if ni >= 0
                            && ni < rows as i32
                            && nj >= 0
                            && nj < cols as i32
                            && grid[ni as usize][nj as usize] == '@'
                        {
                            adjacent_count += 1;
                        }
                    }

                    // Accessible if fewer than 4 adjacent rolls
                    if adjacent_count < 4 {
                        to_remove.push((i, j));
                    }
                }
            }
        }

        // If no rolls can be removed, we're done
        if to_remove.is_empty() {
            break;
        }

        // Remove all accessible rolls
        for (i, j) in &to_remove {
            grid[*i][*j] = '.';
        }

        total_removed += to_remove.len();
    }

    total_removed.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(count_accessible_rolls(input), "13");
    }

    #[test]
    fn test_example_part2() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(count_accessible_rolls_part2(input), "43");
    }
}
