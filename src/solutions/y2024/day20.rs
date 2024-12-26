use std::collections::{BinaryHeap, HashMap};

use crate::solutions::y2024::helpers::coord::Coord;
use crate::solutions::y2024::helpers::grid::Grid;

/// Represents the puzzle state and operations.
struct Puzzle {
    racetrack: Grid,                 // The racetrack
    start: Coord,                    // Start position
    end: Coord,                      // End position
    from_start: HashMap<Coord, i32>, // Distance from start
    to_end: HashMap<Coord, i32>,     // Distance from end
    boring: i32,                     // Total track length without cheats
    track: Vec<Coord>,               // Valid track positions
}

impl Puzzle {
    /// Initializes a new Puzzle instance.
    fn new(grid: Grid, start: Coord, end: Coord) -> Self {
        let mut puzzle = Self {
            racetrack: grid,
            start,
            end,
            from_start: HashMap::new(),
            to_end: HashMap::new(),
            boring: 0,
            track: Vec::new(),
        };
        puzzle.initialize();
        puzzle
    }

    /// Configures the puzzle with distances and track positions.
    fn initialize(&mut self) {
        self.from_start = self.compute_distances(self.start);
        self.to_end = self.compute_distances(self.end);
        self.boring = self.from_start[&self.end];

        self.track = self
            .racetrack
            .iter()
            .filter(|&(pos, _)| self.racetrack[pos] != '#')
            .map(|(pos, _)| pos)
            .collect();
    }

    /// Computes distances from a given start position to all reachable positions.
    fn compute_distances(&self, start: Coord) -> HashMap<Coord, i32> {
        let mut costs = HashMap::new();
        let mut heap = BinaryHeap::new();

        costs.insert(start, 0);
        heap.push((0, start));

        while let Some((cost, p)) = heap.pop() {
            for np in self.racetrack.iter_directions(p) {
                if self.racetrack[np] != '#' {
                    let new_cost = cost + 1;
                    if costs.get(&np).unwrap_or(&i32::MAX) > &new_cost {
                        costs.insert(np, new_cost);
                        heap.push((new_cost, np));
                    }
                }
            }
        }

        costs
    }

    /// Solves the problem with the given cheat constraints.
    fn solve(&self, max_cheats: i32, min_gain: i32) -> u32 {
        let mut nb = 0;

        for cheat_start in &self.track {
            for cheat_end in &self.track {
                let cheat_dist = cheat_start.manhattan_distance(cheat_end);

                if cheat_dist <= max_cheats {
                    let time = match (self.from_start.get(cheat_start), self.to_end.get(cheat_end))
                    {
                        (Some(&start_dist), Some(&end_dist)) => start_dist + cheat_dist + end_dist,
                        _ => {
                            println!(
                                "Missing distances for {:?} or {:?}!",
                                cheat_start, cheat_end
                            );
                            continue;
                        }
                    };

                    if time + min_gain <= self.boring {
                        nb += 1;
                    }
                }
            }
        }
        nb
    }

    /// Solves Part 1.
    pub fn solve_part1(grid: Grid, start: Coord, end: Coord) -> u32 {
        let puzzle = Puzzle::new(grid, start, end);
        puzzle.solve(2, 100)
    }

    /// Solves Part 2.
    pub fn solve_part2(grid: Grid, start: Coord, end: Coord) -> u32 {
        let puzzle = Puzzle::new(grid, start, end);
        puzzle.solve(20, 100)
    }
}

/// Parses input into a Grid and start/end positions.
fn parse_input(input: &str) -> (Grid, Coord, Coord) {
    let grid = Grid::parse(input);
    let mut start = Coord::new(0, 0);
    let mut end = Coord::new(0, 0);

    for (pos, &c) in grid.iter() {
        if c == 'S' {
            start = pos;
        } else if c == 'E' {
            end = pos;
        }
    }

    (grid, start, end)
}

/// Solves Part 1.
pub fn solve_part1(input: &str) -> String {
    let (grid, start, end) = parse_input(input);
    Puzzle::solve_part1(grid, start, end).to_string()
}

/// Solves Part 2.
pub fn solve_part2(input: &str) -> String {
    let (grid, start, end) = parse_input(input);
    Puzzle::solve_part2(grid, start, end).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"
###############
#.............#
#.###########.#
#S#.........#E#
###############
";

        assert_eq!(solve_part1(input), "0");
    }

    #[test]
    fn test_part2() {
        let input = r"
###############
#.............#
#.###########.#
#S#.........#E#
###############
";

        assert_eq!(solve_part2(input), "0");
    }
}
