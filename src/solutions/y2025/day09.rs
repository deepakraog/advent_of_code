use std::collections::{HashMap, VecDeque};

pub fn find_largest_rectangle(input: &str) -> String {
    // Parse all red tile coordinates
    let mut tiles: Vec<(i64, i64)> = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 {
            if let (Ok(x), Ok(y)) = (parts[0].parse::<i64>(), parts[1].parse::<i64>()) {
                tiles.push((x, y));
            }
        }
    }

    // Find the largest rectangle area using any two red tiles as opposite corners
    let mut max_area = 0;
    for i in 0..tiles.len() {
        for j in (i + 1)..tiles.len() {
            let (x1, y1) = tiles[i];
            let (x2, y2) = tiles[j];
            let width = (x1 - x2).abs() + 1;
            let height = (y1 - y2).abs() + 1;
            let area = width * height;
            max_area = max_area.max(area);
        }
    }

    max_area.to_string()
}

// Simple Point type for grid operations
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Simple Grid structure
struct Grid {
    width: i32,
    height: i32,
    data: Vec<i64>,
}

impl Grid {
    fn new(width: i32, height: i32, default: i64) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            data: vec![default; size],
        }
    }

    fn contains(&self, p: Point) -> bool {
        p.x >= 0 && p.x < self.width && p.y >= 0 && p.y < self.height
    }

    fn index(&self, p: Point) -> usize {
        (p.y * self.width + p.x) as usize
    }

    fn get(&self, p: Point) -> i64 {
        if self.contains(p) {
            self.data[self.index(p)]
        } else {
            0
        }
    }

    fn set(&mut self, p: Point, value: i64) {
        if self.contains(p) {
            let idx = self.index(p);
            self.data[idx] = value;
        }
    }
}

const OUTSIDE: i64 = 0;
const INSIDE: i64 = 1;
const UNKNOWN: i64 = 2;

const ORIGIN: Point = Point { x: 0, y: 0 };
const UP: Point = Point { x: 0, y: -1 };
const DOWN: Point = Point { x: 0, y: 1 };
const LEFT: Point = Point { x: -1, y: 0 };
const RIGHT: Point = Point { x: 1, y: 0 };
const ORTHOGONAL: [Point; 4] = [UP, RIGHT, DOWN, LEFT];

fn shrink(tiles: &[(i64, i64)], index: usize) -> HashMap<i64, i32> {
    let mut axis: Vec<i64> = tiles
        .iter()
        .map(|tile| if index == 0 { tile.0 } else { tile.1 })
        .collect();
    axis.push(i64::MIN);
    axis.push(i64::MAX);
    axis.sort_unstable();
    axis.dedup();
    axis.iter()
        .enumerate()
        .map(|(i, &n)| (n, i as i32))
        .collect()
}

fn minmax((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> (i32, i32, i32, i32) {
    (x1.min(x2), y1.min(y2), x1.max(x2), y1.max(y2))
}

pub fn find_largest_rectangle_part2(input: &str) -> String {
    // Parse all red tile coordinates
    let mut tiles: Vec<(i64, i64)> = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 {
            if let (Ok(x), Ok(y)) = (parts[0].parse::<i64>(), parts[1].parse::<i64>()) {
                tiles.push((x, y));
            }
        }
    }

    if tiles.is_empty() {
        return "0".to_string();
    }

    let size = tiles.len();

    // Shrink coordinates to reduce grid size
    let shrink_x = shrink(&tiles, 0);
    let shrink_y = shrink(&tiles, 1);
    let shrunk: Vec<(i32, i32)> = tiles
        .iter()
        .map(|&(x, y)| (shrink_x[&x], shrink_y[&y]))
        .collect();

    let mut area = 0u64;
    let mut todo = VecDeque::from([ORIGIN]);
    let mut grid = Grid::new(shrink_x.len() as i32, shrink_y.len() as i32, UNKNOWN);

    // Mark the boundary (red tiles + connecting lines) as INSIDE
    for i in 0..size {
        let (x1, y1, x2, y2) = minmax(shrunk[i], shrunk[(i + 1) % size]);

        for x in x1..=x2 {
            for y in y1..=y2 {
                grid.set(Point::new(x, y), INSIDE);
            }
        }
    }

    // Flood fill from origin to mark outside areas
    while let Some(point) = todo.pop_front() {
        for next in ORTHOGONAL.map(|o| point + o) {
            if grid.contains(next) && grid.get(next) == UNKNOWN {
                grid.set(next, OUTSIDE);
                todo.push_back(next);
            }
        }
    }

    // Build 2D prefix sum array for fast rectangle sum queries
    // grid[i][j] will contain sum of (value != OUTSIDE) from (0,0) to (i,j)
    for y in 1..grid.height {
        for x in 1..grid.width {
            let point = Point::new(x, y);
            let value = if grid.get(point) != OUTSIDE { 1 } else { 0 };
            let up = grid.get(point + UP);
            let left = grid.get(point + LEFT);
            let up_left = grid.get(point + UP + LEFT);
            grid.set(point, value + up + left - up_left);
        }
    }

    // Check all pairs of red tiles as rectangle corners
    for i in 0..size {
        for j in (i + 1)..size {
            let (x1, y1, x2, y2) = minmax(shrunk[i], shrunk[j]);

            let expected = (x2 - x1 + 1) as i64 * (y2 - y1 + 1) as i64;
            let actual = grid.get(Point::new(x2, y2))
                - grid.get(Point::new(x1 - 1, y2))
                - grid.get(Point::new(x2, y1 - 1))
                + grid.get(Point::new(x1 - 1, y1 - 1));

            if expected == actual {
                let (x1_orig, y1_orig) = tiles[i];
                let (x2_orig, y2_orig) = tiles[j];
                let dx = x1_orig.abs_diff(x2_orig) + 1;
                let dy = y1_orig.abs_diff(y2_orig) + 1;
                area = area.max(dx * dy);
            }
        }
    }

    area.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(find_largest_rectangle(input), "50");
    }

    #[test]
    fn test_example_part2() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(find_largest_rectangle_part2(input), "24");
    }
}
