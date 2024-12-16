use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

const DIRS: [(isize, isize); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)]; // East, North, West, South

// Type alias for the return type of parse_input
type ParsedInput = (Vec<Vec<char>>, (usize, usize), (usize, usize));

/// Parses the input and finds the start and end positions.
fn parse_input(input: &str) -> ParsedInput {
    let mut grid = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (r, line) in input.lines().enumerate() {
        let row: Vec<char> = line.chars().collect();
        if let Some(c) = row.iter().position(|&c| c == 'S') {
            start = (r, c);
        }
        if let Some(c) = row.iter().position(|&c| c == 'E') {
            end = (r, c);
        }
        grid.push(row);
    }

    (grid, start, end)
}

/// Finds the minimum score from `S` to `E` using Dijkstra's algorithm.
fn find_min_score(grid: &[Vec<char>], start: (usize, usize), end: (usize, usize)) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    // Priority queue for Dijkstra's algorithm (min-heap)
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start, 0))); // (score, (x, y), direction)

    // Visited states to avoid revisiting with higher costs
    let mut visited: HashMap<((usize, usize), usize), usize> = HashMap::new();

    while let Some(Reverse((score, (x, y), dir))) = heap.pop() {
        if (x, y) == end {
            return score; // Found the shortest path to the end
        }

        // Check if this state has been visited with a lower score
        if let Some(&visited_score) = visited.get(&((x, y), dir)) {
            if visited_score <= score {
                continue;
            }
        }
        visited.insert(((x, y), dir), score);

        // Move forward
        let (dx, dy) = DIRS[dir];
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx >= 0 && ny >= 0 && nx < rows as isize && ny < cols as isize {
            let nx = nx as usize;
            let ny = ny as usize;
            if grid[nx][ny] != '#' {
                heap.push(Reverse((score + 1, (nx, ny), dir)));
            }
        }

        // Rotate clockwise or counterclockwise
        let cw = (dir + 1) % 4;
        let ccw = (dir + 3) % 4;
        heap.push(Reverse((score + 1000, (x, y), cw)));
        heap.push(Reverse((score + 1000, (x, y), ccw)));
    }

    usize::MAX // No valid path found
}

/// Finds the minimum score and tracks paths from `S` to `E`.
fn find_paths_and_tiles(
    grid: &[Vec<char>],
    start: (usize, usize),
    end: (usize, usize),
) -> (usize, HashSet<(usize, usize)>) {
    let rows = grid.len();
    let cols = grid[0].len();

    // Priority queue for Dijkstra's algorithm (min-heap)
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start, 0))); // (score, (x, y), direction)

    // Visited states to avoid revisiting with higher costs
    let mut visited: HashMap<((usize, usize), usize), usize> = HashMap::new();

    // Backtracking map to track paths
    let mut paths: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    let mut min_score = usize::MAX;

    while let Some(Reverse((score, (x, y), dir))) = heap.pop() {
        if (x, y) == end {
            min_score = min_score.min(score); // Update the minimum score if needed
            continue; // Continue processing to track all best paths
        }

        // Check if this state has been visited with a lower score
        if let Some(&visited_score) = visited.get(&((x, y), dir)) {
            if visited_score <= score {
                continue;
            }
        }
        visited.insert(((x, y), dir), score);

        // Move forward
        let (dx, dy) = DIRS[dir];
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx >= 0 && ny >= 0 && nx < rows as isize && ny < cols as isize {
            let nx = nx as usize;
            let ny = ny as usize;
            if grid[nx][ny] != '#' {
                heap.push(Reverse((score + 1, (nx, ny), dir)));
                paths.entry((nx, ny)).or_default().push((x, y));
            }
        }

        // Rotate clockwise or counterclockwise
        let cw = (dir + 1) % 4;
        let ccw = (dir + 3) % 4;
        heap.push(Reverse((score + 1000, (x, y), cw)));
        heap.push(Reverse((score + 1000, (x, y), ccw)));
    }

    // Backtrack to find all tiles in any best path
    let mut best_tiles = HashSet::new();
    let mut queue = vec![end];
    while let Some(tile) = queue.pop() {
        if !best_tiles.insert(tile) {
            continue; // Already processed
        }
        if let Some(parents) = paths.get(&tile) {
            for &parent in parents {
                queue.push(parent);
            }
        }
    }

    (min_score, best_tiles)
}

/// Solves the problem and returns the lowest score as a `String`.
pub fn solve_part1(input: &str) -> String {
    let (grid, start, end) = parse_input(input);
    let min_score = find_min_score(&grid, start, end);
    min_score.to_string()
}

/// Solves Part 2 and returns the count of best path tiles as a `String`.
pub fn solve_part2(input: &str) -> String {
    let (grid, start, end) = parse_input(input);
    let (_min_score, best_tiles) = find_paths_and_tiles(&grid, start, end);
    best_tiles.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min_score1() {
        let input = r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!(solve_part1(input), "7036");
    }

    #[test]
    fn test_find_min_score2() {
        let input = r"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        assert_eq!(solve_part1(input), "11048");
    }

    #[test]
    fn test_find_paths_and_tiles1() {
        let input = r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!(solve_part2(input), "104");
    }

    #[test]
    fn test_find_paths_and_tiles2() {
        let input = r"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        assert_eq!(solve_part2(input), "132");
    }
}
