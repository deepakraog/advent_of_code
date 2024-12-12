use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: isize,
    y: isize,
}

impl Position {
    pub fn add(&self, other: &Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn count_distinct_positions(input: &str) -> String {
    let grid = parse_input(input);
    let directions = [
        Position { x: 0, y: -1 }, // Up
        Position { x: 1, y: 0 },  // Right
        Position { x: 0, y: 1 },  // Down
        Position { x: -1, y: 0 }, // Left
    ];

    let mut guard_position = find_guard_position(&grid);
    let mut visited_positions = HashSet::new();
    let mut current_direction = 0;

    while !is_border_location(&grid, &guard_position) {
        visited_positions.insert(guard_position);
        let next_position = guard_position.add(&directions[current_direction]);

        if !is_valid_position(&grid, &next_position)
            || grid[next_position.y as usize][next_position.x as usize] == '#'
        {
            current_direction = (current_direction + 1) % 4; // Turn right
        } else {
            guard_position = next_position;
        }
    }

    visited_positions.insert(guard_position);
    visited_positions.len().to_string()
}

pub fn count_trapping_obstruction_positions(input: &str) -> String {
    let grid = parse_input(input);
    let rows = grid.len();
    let cols = grid[0].len();

    let (start_row, start_col) = find_guard_start(&grid);
    let directions = [
        (-1, 0), // Up
        (0, 1),  // Right
        (1, 0),  // Down
        (0, -1), // Left
    ];

    let mut trapping_positions = 0;
    let mut non_trapping_positions = HashSet::new();

    for obs_row in 0..rows {
        for obs_col in 0..cols {
            if grid[obs_row][obs_col] != '.' || non_trapping_positions.contains(&(obs_row, obs_col))
            {
                continue; // Skip non-empty cells or already-checked non-trapping positions
            }

            if causes_guard_loop(
                &grid,
                (start_row, start_col),
                (obs_row, obs_col),
                &directions,
            ) {
                trapping_positions += 1;
            } else {
                non_trapping_positions.insert((obs_row, obs_col));
            }
        }
    }

    trapping_positions.to_string()
}

fn causes_guard_loop(
    grid: &[Vec<char>],
    guard_start: (usize, usize),
    obstacle: (usize, usize),
    directions: &[(isize, isize)],
) -> bool {
    let mut visited_states = HashSet::new();
    let mut position = guard_start;
    let mut direction = 0; // 0=up, 1=right, 2=down, 3=left

    loop {
        let state = (position, direction);
        if !visited_states.insert(state) {
            return true; // Guard is in a loop
        }

        // Calculate the next position
        let next_r = position.0 as isize + directions[direction].0;
        let next_c = position.1 as isize + directions[direction].1;

        // Check if the guard leaves the grid
        if next_r < 0
            || next_r >= grid.len() as isize
            || next_c < 0
            || next_c >= grid[0].len() as isize
        {
            return false; // Guard escapes
        }

        let next_position = (next_r as usize, next_c as usize);

        // Check for obstacles
        if grid[next_position.0][next_position.1] == '#' || next_position == obstacle {
            direction = (direction + 1) % 4; // Turn right
        } else {
            position = next_position; // Move forward
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_guard_start(grid: &[Vec<char>]) -> (usize, usize) {
    for (r, row) in grid.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell == '^' {
                return (r, c);
            }
        }
    }
    panic!("Guard starting position not found!");
}

fn find_guard_position(grid: &[Vec<char>]) -> Position {
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == '^' {
                return Position {
                    x: x as isize,
                    y: y as isize,
                };
            }
        }
    }
    panic!("Guard starting position not found!");
}

fn is_valid_position(grid: &[Vec<char>], position: &Position) -> bool {
    position.y >= 0
        && position.y < grid.len() as isize
        && position.x >= 0
        && position.x < grid[0].len() as isize
}

fn is_border_location(grid: &[Vec<char>], position: &Position) -> bool {
    position.x == 0
        || position.y == 0
        || position.x == (grid[0].len() as isize - 1)
        || position.y == (grid.len() as isize - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_distinct_positions() {
        let input = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";
        assert_eq!(count_distinct_positions(input), "41");
    }

    #[test]
    fn test_count_trapping_obstruction_positions() {
        let input = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";
        assert_eq!(count_trapping_obstruction_positions(input), "6");
    }
}
