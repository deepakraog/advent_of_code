pub fn count_xmas(input: &str) -> String {
    let grid = parse_input(input);

    let directions = [
        (0, 1),   // Right
        (1, 0),   // Down
        (1, 1),   // Diagonal down-right
        (1, -1),  // Diagonal down-left
        (0, -1),  // Left
        (-1, 0),  // Up
        (-1, -1), // Diagonal up-left
        (-1, 1),  // Diagonal up-right
    ];

    let word = ['X', 'M', 'A', 'S'];
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // Helper function to check if a word exists in a given direction
    fn is_word_found(
        grid: &[Vec<char>],
        start_row: isize,
        start_col: isize,
        direction: (isize, isize),
        word: &[char],
    ) -> bool {
        let mut row = start_row;
        let mut col = start_col;

        for &c in word {
            if row < 0 || col < 0 || row >= grid.len() as isize || col >= grid[0].len() as isize {
                return false; // Out of bounds
            }
            if grid[row as usize][col as usize] != c {
                return false; // Character mismatch
            }
            row += direction.0;
            col += direction.1;
        }

        true
    }

    // Iterate through each cell in the grid
    for row in 0..rows {
        for col in 0..cols {
            for &direction in &directions {
                if is_word_found(&grid, row as isize, col as isize, direction, &word) {
                    count += 1;
                }
            }
        }
    }

    count.to_string()
}

pub fn count_patterns(input: &str) -> String {
    let grid = parse_input(input);

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            // Check for X-MAS patterns
            if r + 2 < rows && c + 2 < cols {
                // Pattern 1: Forward X-MAS
                if grid[r][c] == 'M'
                    && grid[r + 1][c + 1] == 'A'
                    && grid[r + 2][c + 2] == 'S'
                    && grid[r + 2][c] == 'M'
                    && grid[r][c + 2] == 'S'
                {
                    count += 1;
                }
                // Pattern 2: Reverse X-MAS
                if grid[r][c] == 'M'
                    && grid[r + 1][c + 1] == 'A'
                    && grid[r + 2][c + 2] == 'S'
                    && grid[r + 2][c] == 'S'
                    && grid[r][c + 2] == 'M'
                {
                    count += 1;
                }
                // Pattern 3: Forward X-SAM
                if grid[r][c] == 'S'
                    && grid[r + 1][c + 1] == 'A'
                    && grid[r + 2][c + 2] == 'M'
                    && grid[r + 2][c] == 'M'
                    && grid[r][c + 2] == 'S'
                {
                    count += 1;
                }
                // Pattern 4: Reverse X-SAM
                if grid[r][c] == 'S'
                    && grid[r + 1][c + 1] == 'A'
                    && grid[r + 2][c + 2] == 'M'
                    && grid[r + 2][c] == 'S'
                    && grid[r][c + 2] == 'M'
                {
                    count += 1;
                }
            }
        }
    }

    count.to_string()
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_patterns_simple() {
        let input = "M.S\n.A.\nM.S".to_string();
        assert_eq!(count_patterns(&input), "1");
    }

    #[test]
    fn test_count_patterns_complex() {
        let input = ".M.S......\n..A..MSMS.\n.M.S.MAA..\n..A.ASMSM.\n.M.S.M....\n..........\
                     \nS.S.S.S.S.\n.A.A.A.A..\nM.M.M.M.M.\n.........."
            .to_string();
        assert_eq!(count_patterns(&input), "9");
    }

    #[test]
    fn test_count_xmas_patterns_simple() {
        let input = "M.S\n.A.\nM.S".to_string();
        assert_eq!(count_patterns(&input), "1");
    }

    #[test]
    fn test_count_xmas_patterns_complex() {
        let input = ".M.S......\n..A..MSMS.\n.M.S.MAA..\n..A.ASMSM.\n.M.S.M....\n..........\
                     \nS.S.S.S.S.\n.A.A.A.A..\nM.M.M.M.M.\n.........."
            .to_string();
        assert_eq!(count_patterns(&input), "9");
    }
}
