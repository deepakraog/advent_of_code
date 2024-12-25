/// Type alias for a grid of characters.
type Grid = Vec<Vec<char>>;

/// Type alias for a collection of keys and locks.
type KeysAndLocks = (Vec<Grid>, Vec<Grid>);

/// Parses the input into keys and locks.
fn parse_input(input: &str) -> KeysAndLocks {
    let shapes: Vec<&str> = input.trim().split("\n\n").collect();
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for shape in shapes {
        let grid: Grid = shape.lines().map(|line| line.chars().collect()).collect();
        if grid[0].iter().all(|&cell| cell == '.') {
            keys.push(grid);
        } else {
            locks.push(grid);
        }
    }

    (keys, locks)
}

/// Checks if a key fits into a lock.
fn fits(key: &Grid, lock: &Grid) -> bool {
    for (key_row, lock_row) in key.iter().zip(lock.iter()) {
        for (&key_cell, &lock_cell) in key_row.iter().zip(lock_row.iter()) {
            if key_cell == '#' && lock_cell == '#' {
                return false;
            }
        }
    }
    true
}

/// Finds all valid key/lock pairs.
fn find_valid_pairs(keys: &[Grid], locks: &[Grid]) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();

    for (key_idx, key) in keys.iter().enumerate() {
        for (lock_idx, lock) in locks.iter().enumerate() {
            if fits(key, lock) {
                pairs.push((key_idx, lock_idx));
            }
        }
    }

    pairs
}

/// Solves Part 1: Counts the number of valid key/lock pairs.
pub fn solve_part1(input: &str) -> String {
    let (keys, locks) = parse_input(input);
    find_valid_pairs(&keys, &locks).len().to_string()
}

/// Solves Part 2: Finds all valid key/lock pairs and returns a detailed report.
pub fn solve_part2(input: &str) -> String {
    let (keys, locks) = parse_input(input);
    let valid_pairs = find_valid_pairs(&keys, &locks);

    let mut report = String::new();
    report.push_str(&format!("Total Valid Pairs: {}\n", valid_pairs.len()));
    for (key_idx, lock_idx) in valid_pairs {
        report.push_str(&format!("Key {} fits Lock {}\n", key_idx, lock_idx));
    }

    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = r"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####";

        assert_eq!(solve_part1(input), "1");
    }

    #[test]
    fn test_part2_example() {
        let input = r"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####";

        let expected_report = "Total Valid Pairs: 1\nKey 1 fits Lock 1\n";
        assert_eq!(solve_part2(input), expected_report);
    }
}
