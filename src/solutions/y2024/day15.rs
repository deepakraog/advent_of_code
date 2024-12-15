use std::collections::{HashSet, VecDeque};

/// Parses the input and returns the grid and instructions.
fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let mut parts = input.split("\n\n");
    let grid: Vec<Vec<char>> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let instructions: Vec<char> = parts
        .next()
        .unwrap()
        .chars()
        .filter(|&c| "^v<>".contains(c))
        .collect();

    (grid, instructions)
}

/// Solves the warehouse simulation.
fn solve(input: &str, part2: bool) -> String {
    let (mut grid, instructions) = parse_input(input);

    let rows = grid.len();
    let cols = grid[0].len();

    // Expand grid for part 2
    if part2 {
        let mut big_grid = vec![vec!['.'; cols * 2]; rows];
        for r in 0..rows {
            for c in 0..cols {
                match grid[r][c] {
                    '#' => {
                        big_grid[r][c * 2] = '#';
                        big_grid[r][c * 2 + 1] = '#';
                    }
                    'O' => {
                        big_grid[r][c * 2] = '[';
                        big_grid[r][c * 2 + 1] = ']';
                    }
                    '@' => {
                        big_grid[r][c * 2] = '@';
                        big_grid[r][c * 2 + 1] = '.';
                    }
                    '.' => {
                        big_grid[r][c * 2] = '.';
                        big_grid[r][c * 2 + 1] = '.';
                    }
                    _ => (),
                }
            }
        }
        grid = big_grid;
    }

    let mut robot_pos = (0, 0);
    let rows = grid.len();
    let cols = grid[0].len();

    // Find the initial robot position
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '@' {
                robot_pos = (r, c);
                grid[r][c] = '.';
            }
        }
    }

    let directions = vec![('^', (-1, 0)), ('v', (1, 0)), ('<', (0, -1)), ('>', (0, 1))];
    let mut r = robot_pos.0;
    let mut c = robot_pos.1;

    for &inst in &instructions {
        let (dr, dc) = directions.iter().find(|&&(ch, _)| ch == inst).unwrap().1;
        let rr = (r as isize + dr) as usize;
        let cc = (c as isize + dc) as usize;

        if grid[rr][cc] == '#' {
            continue; // Wall, no movement
        } else if grid[rr][cc] == '.' {
            r = rr;
            c = cc;
        } else if grid[rr][cc] == '[' || grid[rr][cc] == ']' || grid[rr][cc] == 'O' {
            let mut queue = VecDeque::new();
            let mut seen = HashSet::new();
            queue.push_back((r, c));
            let mut ok = true;

            while let Some((cr, cc)) = queue.pop_front() {
                if seen.contains(&(cr, cc)) {
                    continue;
                }
                seen.insert((cr, cc));
                let nrr = (cr as isize + dr) as usize;
                let ncc = (cc as isize + dc) as usize;

                if grid[nrr][ncc] == '#' {
                    ok = false;
                    break;
                }
                match grid[nrr][ncc] {
                    'O' => queue.push_back((nrr, ncc)),
                    '[' => {
                        queue.push_back((nrr, ncc));
                        if grid[nrr][ncc + 1] == ']' {
                            queue.push_back((nrr, ncc + 1));
                        }
                    }
                    ']' => {
                        queue.push_back((nrr, ncc));
                        if grid[nrr][ncc - 1] == '[' {
                            queue.push_back((nrr, ncc - 1));
                        }
                    }
                    '.' => {}
                    _ => (),
                }
            }

            if !ok {
                continue;
            }

            while !seen.is_empty() {
                let mut to_remove = vec![];
                for &(cr, cc) in &seen {
                    let nrr = (cr as isize + dr) as usize;
                    let ncc = (cc as isize + dc) as usize;

                    if !seen.contains(&(nrr, ncc)) {
                        assert_eq!(grid[nrr][ncc], '.');
                        grid[nrr][ncc] = grid[cr][cc];
                        grid[cr][cc] = '.';
                        to_remove.push((cr, cc));
                    }
                }

                for pos in to_remove {
                    seen.remove(&pos);
                }
            }

            r = (r as isize + dr) as usize;
            c = (c as isize + dc) as usize;
        }
    }

    // Calculate GPS coordinate sum
    let mut gps_sum = 0;
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '[' || grid[r][c] == 'O' {
                gps_sum += 100 * r + c;
            }
        }
    }
    gps_sum.to_string()
}

pub fn solve_part1(input: &str) -> String {
    solve(input, false)
}

pub fn solve_part2(input: &str) -> String {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_example() {
        let input = r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

        assert_eq!(solve_part1(input), "2028");
        // Replace <expected_result_part2> with the correct value for part 2
        assert_eq!(solve_part2(input), "<expected_result_part2>");
    }
}
