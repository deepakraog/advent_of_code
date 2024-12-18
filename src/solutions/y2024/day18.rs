use std::collections::{HashSet, VecDeque};

/// Parses the input into a vector of byte positions.
fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let mut coords = line.split(',').map(|x| x.parse::<usize>().unwrap());
            (coords.next().unwrap(), coords.next().unwrap())
        })
        .collect()
}

/// Performs a Breadth-First Search (BFS) to determine if there is a path from the top-left
/// to the bottom-right corner of the grid.
fn bfs_path_exists(grid: &[Vec<bool>], start: (usize, usize), goal: (usize, usize)) -> bool {
    let grid_size = grid.len();
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some((x, y)) = queue.pop_front() {
        if (x, y) == goal {
            return true; // Found a path
        }

        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0
                && ny >= 0
                && (nx as usize) < grid_size
                && (ny as usize) < grid_size
                && !grid[ny as usize][nx as usize]
                && !visited.contains(&(nx as usize, ny as usize))
            {
                visited.insert((nx as usize, ny as usize));
                queue.push_back((nx as usize, ny as usize));
            }
        }
    }

    false // No path exists
}

/// Simulates the corrupted grid and calculates the shortest path.
fn find_shortest_path(grid_size: usize, bytes: &[(usize, usize)]) -> Option<usize> {
    let mut grid = vec![vec![false; grid_size]; grid_size];

    // Corrupt memory according to the bytes
    for &(x, y) in bytes.iter().take(1024) {
        grid[y][x] = true;
    }

    // Perform BFS to find the shortest path
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((0, 0, 0)); // (x, y, steps)
    visited.insert((0, 0));

    while let Some((x, y, steps)) = queue.pop_front() {
        if x == grid_size - 1 && y == grid_size - 1 {
            return Some(steps); // Found the shortest path
        }

        for &(dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0
                && ny >= 0
                && (nx as usize) < grid_size
                && (ny as usize) < grid_size
                && !grid[ny as usize][nx as usize]
                && !visited.contains(&(nx as usize, ny as usize))
            {
                visited.insert((nx as usize, ny as usize));
                queue.push_back((nx as usize, ny as usize, steps + 1));
            }
        }
    }

    None // No path found
}

/// Finds the first byte that blocks the path from (0, 0) to (grid_size-1, grid_size-1).
fn find_blocking_byte(grid_size: usize, bytes: &[(usize, usize)]) -> Option<(usize, usize)> {
    let mut grid = vec![vec![false; grid_size]; grid_size];

    for &(x, y) in bytes {
        grid[y][x] = true;

        // Check if the path is blocked
        if !bfs_path_exists(&grid, (0, 0), (grid_size - 1, grid_size - 1)) {
            return Some((x, y));
        }
    }

    None // No blocking byte found
}

/// Solves Part 1.
pub fn solve_part1(input: &str) -> String {
    let grid_size = 71; // Grid size for 0 to 70 inclusive
    let bytes = parse_input(input);

    match find_shortest_path(grid_size, &bytes) {
        Some(steps) => steps.to_string(),
        None => "No path to the exit".to_string(),
    }
}

/// Solves Part 2.
pub fn solve_part2(input: &str) -> String {
    let grid_size = 71; // Grid size for 0 to 70 inclusive
    let bytes = parse_input(input);

    match find_blocking_byte(grid_size, &bytes) {
        Some((x, y)) => format!("{},{}", x, y),
        None => "No blocking byte found".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case_part1() {
        let input = r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!(solve_part1(input), "22");
    }

    #[test]
    fn test_example_case_part2() {
        let input = r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!(solve_part2(input), "6,1");
    }
}
