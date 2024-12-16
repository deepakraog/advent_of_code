use std::collections::{HashMap, HashSet};

/// Represents a robot with position and velocity.
#[derive(Debug, Clone, Copy)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

/// Parses the input and returns a list of robots.
fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let position: Vec<i32> = parts[0]
                .trim_start_matches("p=")
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect();
            let velocity: Vec<i32> = parts[1]
                .trim_start_matches("v=")
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect();

            Robot {
                position: (position[0], position[1]),
                velocity: (velocity[0], velocity[1]),
            }
        })
        .collect()
}

/// Simulates the robot positions after a given amount of time.
fn simulate_positions(robots: &[Robot], time: i32) -> HashMap<(i32, i32), i32> {
    let mut positions = HashMap::new();

    for robot in robots {
        let new_x = (robot.position.0 + robot.velocity.0 * time).rem_euclid(101);
        let new_y = (robot.position.1 + robot.velocity.1 * time).rem_euclid(103);

        *positions.entry((new_x, new_y)).or_insert(0) += 1;
    }

    positions
}

/// Divides the space into quadrants and calculates the robot count in each quadrant.
fn count_robots_in_quadrants(positions: &HashMap<(i32, i32), i32>) -> [i32; 4] {
    let mut quadrants = [0; 4];

    for (&(x, y), &count) in positions {
        if x == 101 / 2 || y == 103 / 2 {
            continue; // Robots in the middle don't count
        }

        let quadrant = if x < 101 / 2 && y < 103 / 2 {
            0 // Top-left
        } else if x >= 101 / 2 && y < 103 / 2 {
            1 // Top-right
        } else if x < 101 / 2 && y >= 103 / 2 {
            2 // Bottom-left
        } else {
            3 // Bottom-right
        };

        quadrants[quadrant] += count;
    }

    quadrants
}

/// Computes the safety factor by multiplying robot counts in each quadrant.
fn compute_safety_factor(quadrants: [i32; 4]) -> i32 {
    quadrants.iter().product()
}

/// Solves Part 1 of the puzzle.
pub fn solve_part1(input: &str) -> String {
    let robots = parse_input(input);
    let positions = simulate_positions(&robots, 100);
    let quadrants = count_robots_in_quadrants(&positions);
    compute_safety_factor(quadrants).to_string()
}

/// Simulates robot positions at a specific time step.
fn simulate_positions_set(robots: &[Robot], time: i32) -> HashSet<(i32, i32)> {
    robots
        .iter()
        .map(|robot| {
            let new_x = robot.position.0 + robot.velocity.0 * time;
            let new_y = robot.position.1 + robot.velocity.1 * time;
            (new_x, new_y)
        })
        .collect()
}

/// Finds the bounding box of the given positions.
fn bounding_box(positions: &HashSet<(i32, i32)>) -> ((i32, i32), (i32, i32)) {
    let min_x = positions.iter().map(|&(x, _)| x).min().unwrap();
    let max_x = positions.iter().map(|&(x, _)| x).max().unwrap();
    let min_y = positions.iter().map(|&(_, y)| y).min().unwrap();
    let max_y = positions.iter().map(|&(_, y)| y).max().unwrap();

    ((min_x, min_y), (max_x, max_y))
}

/// Visualizes the positions of robots on a grid.
fn visualize_positions(positions: &HashSet<(i32, i32)>) -> String {
    let ((min_x, min_y), (max_x, max_y)) = bounding_box(positions);
    let mut result = String::new();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if positions.contains(&(x, y)) {
                result.push('#');
            } else {
                result.push('.');
            }
        }
        result.push('\n');
    }

    result
}

/// Solves Part 2: Finds the fewest seconds for robots to display the Easter egg.
pub fn solve_part2(input: &str) -> String {
    let robots = parse_input(input);

    let mut time = 0;
    let mut last_area = i32::MAX;
    let mut last_positions = HashSet::new();

    loop {
        let positions = simulate_positions_set(&robots, time);
        let ((min_x, min_y), (max_x, max_y)) = bounding_box(&positions);
        let area = (max_x - min_x + 1) * (max_y - min_y + 1);

        if area > last_area {
            // When the bounding box starts expanding, return the last valid visualization
            let visualization = visualize_positions(&last_positions);
            return format!("Time: {}\n{}", time - 1, visualization);
        }

        last_area = area;
        last_positions = positions;
        time += 1;

        // Safety condition to prevent infinite loops
        if time > 10_000 {
            return "No clear Easter egg found within 10,000 seconds.".to_string();
        }
    }
}
