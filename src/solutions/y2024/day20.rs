use std::collections::{HashMap, HashSet, VecDeque};

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)]; // Directions: up, right, down, left

type ParseResult = (Vec<Vec<char>>, (usize, usize), (usize, usize));

/// Parses the input and converts it into a grid with start and end positions.
fn parse_input(input: &str) -> ParseResult {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (r, row) in grid.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                start = (r, c);
            } else if cell == 'E' {
                end = (r, c);
            }
        }
    }

    (grid, start, end)
}

/// Performs BFS to calculate distances from the endpoint to all reachable cells.
fn bfs(grid: &[Vec<char>], end: (usize, usize)) -> HashMap<(usize, usize), i32> {
    let mut dist = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((0, end.0, end.1));

    while let Some((d, r, c)) = queue.pop_front() {
        if dist.contains_key(&(r, c)) {
            continue;
        }
        dist.insert((r, c), d);

        for &(dr, dc) in &DIRS {
            let nr = r as isize + dr;
            let nc = c as isize + dc;

            if nr >= 0
                && nc >= 0
                && (nr as usize) < grid.len()
                && (nc as usize) < grid[0].len()
                && grid[nr as usize][nc as usize] != '#'
            {
                queue.push_back((d + 1, nr as usize, nc as usize));
            }
        }
    }

    dist
}

/// Simulates the race with cheats and finds the total number of valid cheats.
fn find_cheat(
    grid: &[Vec<char>],
    dist: &HashMap<(usize, usize), i32>,
    start: (usize, usize),
    max_distance: i32,
    cheat_time: usize,
) -> usize {
    let mut seen = HashSet::new(); // Tracks visited states
    let mut queue = VecDeque::new();
    let mut cheat_count = 0;

    queue.push_back((0, false, cheat_time, start.0, start.1));

    while let Some((d, used_cheat, remaining_cheat, r, c)) = queue.pop_front() {
        if !seen.insert((d, used_cheat, remaining_cheat, r, c)) {
            continue; // Skip already visited states
        }

        if let Some(&target_dist) = dist.get(&(r, c)) {
            if !used_cheat && target_dist <= max_distance - d - 100 {
                cheat_count += 1;
            }
        }

        for &(dr, dc) in &DIRS {
            let nr = r as isize + dr;
            let nc = c as isize + dc;

            if nr < 0 || nc < 0 || nr as usize >= grid.len() || nc as usize >= grid[0].len() {
                continue;
            }

            let nr = nr as usize;
            let nc = nc as usize;

            if grid[nr][nc] != '#' {
                queue.push_back((d + 1, used_cheat, remaining_cheat, nr, nc));
            } else if !used_cheat && remaining_cheat > 0 {
                queue.push_back((d + 1, true, remaining_cheat - 1, nr, nc));
            }
        }
    }

    cheat_count
}

/// Solves Part 1 by finding cheats with a cheat duration of 2.
pub fn solve_part1(input: &str) -> String {
    let (grid, start, end) = parse_input(input);
    let dist = bfs(&grid, end);
    let max_distance = *dist.get(&start).unwrap();

    find_cheat(&grid, &dist, start, max_distance, 2).to_string()
}

/// Solves Part 2 by finding cheats with a cheat duration of 20.
pub fn solve_part2(input: &str) -> String {
    let (grid, start, end) = parse_input(input);
    let dist = bfs(&grid, end);
    let max_distance = *dist.get(&start).unwrap();

    find_cheat(&grid, &dist, start, max_distance, 20).to_string()
}
