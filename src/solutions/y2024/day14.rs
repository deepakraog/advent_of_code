use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

struct Puzzle {
    robots: Vec<Robot>,
    width: i32,
    height: i32,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            robots: Vec::new(),
            width: 101,
            height: 103,
        }
    }

    fn configure(&mut self, input: &str) {
        let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

        for line in input.lines() {
            let caps = re.captures(line).unwrap();

            let robot = Robot {
                px: caps.get(1).unwrap().as_str().parse().unwrap(),
                py: caps.get(2).unwrap().as_str().parse().unwrap(),
                vx: caps.get(3).unwrap().as_str().parse().unwrap(),
                vy: caps.get(4).unwrap().as_str().parse().unwrap(),
            };

            self.robots.push(robot);
        }

        if input.contains("test") {
            self.width = 11;
            self.height = 7;
        }
    }

    fn solve_part1(&self) -> u32 {
        let mut quadrants = HashMap::new();

        for robot in &self.robots {
            let px = (robot.px + robot.vx * 100).rem_euclid(self.width);
            let py = (robot.py + robot.vy * 100).rem_euclid(self.height);

            if px == self.width / 2 || py == self.height / 2 {
                continue;
            }

            let q = ((px * 2) / self.width, (py * 2) / self.height);
            *quadrants.entry(q).or_default() += 1_u32;
        }

        quadrants.values().product::<u32>()
    }

    fn solve_part2(&self) -> i32 {
        'outer: for seconds in 0..100_000 {
            let mut grid: HashMap<(i32, i32), u32> = HashMap::new();

            for robot in &self.robots {
                let px = (robot.px + robot.vx * seconds).rem_euclid(self.width);
                let py = (robot.py + robot.vy * seconds).rem_euclid(self.height);

                if grid.contains_key(&(px, py)) {
                    continue 'outer;
                }

                *grid.entry((px, py)).or_default() += 1;
            }

            return seconds;
        }

        0
    }
}

/// Solves Part 1: Computes the safety factor of the robots.
pub fn solve_part1(input: &str) -> String {
    let mut puzzle = Puzzle::new();
    puzzle.configure(input);
    puzzle.solve_part1().to_string()
}

/// Solves Part 2: Finds the time at which robots align to form a pattern.
pub fn solve_part2(input: &str) -> String {
    let mut puzzle = Puzzle::new();
    puzzle.configure(input);
    puzzle.solve_part2().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"p=3,1 v=1,0
p=2,2 v=1,-1
p=3,3 v=-1,0
p=4,4 v=0,1";
        assert_eq!(solve_part1(input), "4");
    }

    #[test]
    fn test_part2() {
        let input = r"p=3,1 v=1,0
p=2,2 v=1,-1
p=3,3 v=-1,0
p=4,4 v=0,1";
        assert_eq!(solve_part2(input), "0");
    }
}
