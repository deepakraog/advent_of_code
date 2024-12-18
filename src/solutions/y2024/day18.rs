use std::collections::{HashSet, VecDeque};

/// Parses the input into a vector of byte positions.
fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let coords: Vec<usize> = line.split(',').map(|x| x.parse().unwrap()).collect();
            (coords[0], coords[1])
        })
        .collect()
}

/// Simulates the corrupted grid after bytes fall and returns the grid.
fn simulate_corruption(grid_size: usize, bytes: &[(usize, usize)]) -> Vec<Vec<bool>> {
    let mut grid = vec![vec![false; grid_size]; grid_size];

    for &(x, y) in bytes.iter().take(1024) {
        grid[y][x] = true; // Mark cell as corrupted
    }

    grid
}

/// Finds the shortest path from (0, 0) to (grid_size-1, grid_size-1) avoiding corrupted cells.
fn find_shortest_path(grid: &[Vec<bool>]) -> Option<usize> {
    let grid_size = grid.len();
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    // Start BFS from the top-left corner
    queue.push_back((0, 0, 0)); // (x, y, steps)
    visited.insert((0, 0));

    while let Some((x, y, steps)) = queue.pop_front() {
        if x == grid_size - 1 && y == grid_size - 1 {
            return Some(steps); // Reached the bottom-right corner
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
                queue.push_back((nx as usize, ny as usize, steps + 1));
            }
        }
    }

    None // No path found
}

/// Solves the problem for Part 1.
pub fn solve_part1(input: &str) -> String {
    let grid_size = 71; // Grid size for 0 to 70 inclusive
    let bytes = parse_input(input);

    let grid = simulate_corruption(grid_size, &bytes);
    match find_shortest_path(&grid) {
        Some(steps) => steps.to_string(),
        None => "No path to the exit".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case() {
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
        assert_eq!(solve_part1(input), "146");
    }

    #[test]
    fn test_no_path_case() {
        let input = r"0,1
1,0
1,1
0,2
2,0";
        assert_eq!(solve_part1(input), "No path to the exit");
    }
}
