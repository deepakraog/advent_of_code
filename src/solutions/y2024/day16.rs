use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::coord::Coord;

const ZERO: Coord = Coord { x: 0, y: 0 };
const EAST: Coord = Coord { x: 1, y: 0 }; // starting direction

struct Cost1 {
    cost: u32,
    pos: Coord,
    dir: Coord,
}

impl Cost1 {
    const fn new(cost: u32, pos: Coord, dir: Coord) -> Self {
        Self { cost, pos, dir }
    }
}

impl Ord for Cost1 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Cost1 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cost1 {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Cost1 {}

struct Cost2 {
    cost: u32,
    pos: Coord,
    dir: Coord,
    path: Vec<Coord>,
}

impl Cost2 {
    const fn new(cost: u32, pos: Coord, dir: Coord, path: Vec<Coord>) -> Self {
        Self {
            cost,
            pos,
            dir,
            path,
        }
    }
}

impl Ord for Cost2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Cost2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cost2 {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Cost2 {}

struct Puzzle {
    start: Coord,
    end: Coord,
    maze: HashSet<Coord>,
    size: Coord,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            start: ZERO,
            end: ZERO,
            maze: HashSet::new(),
            size: ZERO,
        }
    }

    fn configure(&mut self, input: &str) {
        for (y, line) in input.lines().enumerate() {
            let y = i32::try_from(y).unwrap();

            for (x, c) in line.chars().enumerate() {
                let x = i32::try_from(x).unwrap();
                self.size.x = x;

                if c == '#' {
                    continue;
                }

                if c == 'S' {
                    self.start = Coord::new(x, y);
                } else if c == 'E' {
                    self.end = Coord::new(x, y);
                }
                self.maze.insert(Coord { x, y });
            }

            self.size.y = y;
        }
    }

    fn part1(&self) -> u32 {
        let mut seen = HashSet::new();
        let mut heap = BinaryHeap::new();

        heap.push(Cost1::new(0, self.start, EAST));

        while let Some(Cost1 { cost, pos, dir }) = heap.pop() {
            seen.insert((pos, dir));

            let counterclockwise = Coord::new(dir.y, -dir.x);
            let clockwise = Coord::new(-dir.y, dir.x);

            for (new_cost, new_pos, new_dir) in [
                (cost + 1, pos + dir, dir),
                (cost + 1001, pos + counterclockwise, counterclockwise),
                (cost + 1001, pos + clockwise, clockwise),
            ] {
                if new_pos == self.end {
                    return new_cost;
                }
                if self.maze.contains(&new_pos) && !seen.contains(&(new_pos, dir)) {
                    heap.push(Cost1::new(new_cost, new_pos, new_dir));
                }
            }
        }

        0
    }

    fn part2(&self) -> usize {
        let mut heap = BinaryHeap::new();
        let mut costs = HashMap::new();
        let mut best_path_tiles = HashSet::new();

        heap.push(Cost2::new(0, self.start, EAST, vec![self.start]));

        while let Some(Cost2 {
            cost,
            pos,
            dir,
            path: tiles,
        }) = heap.pop()
        {
            if pos == self.end {
                best_path_tiles.extend(tiles.clone());
            }

            let counterclockwise = Coord::new(dir.y, -dir.x);
            let clockwise = Coord::new(-dir.y, dir.x);

            for (new_cost, new_pos, new_dir) in [
                (cost + 1, pos + dir, dir),
                (cost + 1001, pos + counterclockwise, counterclockwise),
                (cost + 1001, pos + clockwise, clockwise),
            ] {
                if self.maze.contains(&new_pos)
                    && costs.get(&(new_pos, new_dir)).copied().unwrap_or(u32::MAX) >= new_cost
                {
                    costs.insert((new_pos, new_dir), new_cost);
                    let mut new_tiles = tiles.clone();
                    new_tiles.push(new_pos);
                    heap.push(Cost2::new(new_cost, new_pos, new_dir, new_tiles));
                }
            }
        }

        best_path_tiles.len()
    }
}

pub fn solve_part1(input: &str) -> String {
    let mut puzzle = Puzzle::new();
    puzzle.configure(input);
    puzzle.part1().to_string()
}

pub fn solve_part2(input: &str) -> String {
    let mut puzzle = Puzzle::new();
    puzzle.configure(input);
    puzzle.part2().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"S.....
..####
.....
....E";
        assert_eq!(solve_part1(input), "2007");
    }

    #[test]
    fn test_part2() {
        let input = r"S.....
..####
.....
....E";
        assert_eq!(solve_part2(input), "14");
    }
}
