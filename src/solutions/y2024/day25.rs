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

/// Counts the number of valid key/lock pairs.
fn count_valid_pairs(keys: &[Grid], locks: &[Grid]) -> usize {
    let mut count = 0;

    for key in keys {
        for lock in locks {
            if fits(key, lock) {
                count += 1;
            }
        }
    }

    count
}

/// Solves Part 1: Counts the number of valid key/lock pairs.
pub fn solve_part1(input: &str) -> String {
    let (keys, locks) = parse_input(input);
    count_valid_pairs(&keys, &locks).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
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
}
