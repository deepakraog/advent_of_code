pub fn calculate_total_fence_price(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let n = grid.len();
    let m = grid[0].len();

    let mut visited = vec![vec![false; m]; n];
    let mut total_price = 0;

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for i in 0..n {
        for j in 0..m {
            if !visited[i][j] {
                let plant_type = grid[i][j];
                let (area, perimeter) =
                    bfs_region(&grid, &mut visited, (i, j), plant_type, &directions);
                total_price += area * perimeter;
            }
        }
    }

    total_price.to_string()
}

fn bfs_region(
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    start: (usize, usize),
    plant_type: char,
    directions: &[(isize, isize)],
) -> (u32, u32) {
    let n = grid.len();
    let m = grid[0].len();

    let mut queue = std::collections::VecDeque::new();
    let mut region_area = 0;
    let mut region_perimeter = 0;

    queue.push_back(start);
    visited[start.0][start.1] = true;

    while let Some((x, y)) = queue.pop_front() {
        region_area += 1;

        for &(dx, dy) in directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < n as isize && ny >= 0 && ny < m as isize {
                let nx = nx as usize;
                let ny = ny as usize;

                if grid[nx][ny] == plant_type && !visited[nx][ny] {
                    visited[nx][ny] = true;
                    queue.push_back((nx, ny));
                } else if grid[nx][ny] != plant_type {
                    region_perimeter += 1;
                }
            } else {
                // Out of bounds, counts as perimeter
                region_perimeter += 1;
            }
        }
    }

    (region_area, region_perimeter)
}

use std::collections::{HashMap, HashSet, VecDeque};

pub fn calculate_total_fence_price_with_sides(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let n = grid.len();
    let m = grid[0].len();

    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)]; // up, right, down, left
    let mut seen = HashSet::new();
    let mut total_price = 0;

    for r in 0..n {
        for c in 0..m {
            if seen.contains(&(r, c)) {
                continue;
            }

            let mut queue = VecDeque::new();
            queue.push_back((r, c));
            let mut area = 0;

            let mut perim_map: HashMap<(isize, isize), HashSet<(usize, usize)>> = HashMap::new();

            while let Some((r2, c2)) = queue.pop_front() {
                if seen.contains(&(r2, c2)) {
                    continue;
                }

                seen.insert((r2, c2));
                area += 1;

                for &(dr, dc) in &directions {
                    let rr = r2 as isize + dr;
                    let cc = c2 as isize + dc;

                    if rr >= 0 && rr < n as isize && cc >= 0 && cc < m as isize {
                        let rr = rr as usize;
                        let cc = cc as usize;

                        if grid[rr][cc] == grid[r2][c2] {
                            queue.push_back((rr, cc));
                        } else {
                            perim_map.entry((dr, dc)).or_default().insert((r2, c2));
                        }
                    } else {
                        perim_map.entry((dr, dc)).or_default().insert((r2, c2));
                    }
                }
            }

            // Calculate sides
            let mut sides = 0;
            for (_, vs) in perim_map.iter() {
                let mut seen_perim = HashSet::new();
                let mut queue = VecDeque::new();

                for &(pr, pc) in vs {
                    if !seen_perim.contains(&(pr, pc)) {
                        sides += 1;
                        queue.push_back((pr, pc));

                        while let Some((r2, c2)) = queue.pop_front() {
                            if seen_perim.contains(&(r2, c2)) {
                                continue;
                            }

                            seen_perim.insert((r2, c2));

                            for &(dr, dc) in &directions {
                                let rr = r2 as isize + dr;
                                let cc = c2 as isize + dc;

                                if vs.contains(&(rr as usize, cc as usize)) {
                                    queue.push_back((rr as usize, cc as usize));
                                }
                            }
                        }
                    }
                }
            }

            total_price += area * sides;
        }
    }

    total_price.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "AAAA\nBBCD\nBBCC\nEEEC".to_string();
        assert_eq!(calculate_total_fence_price_with_sides(&input), "80");
    }

    #[test]
    fn test_example_2() {
        let input = "EEEEE\nEXXXX\nEEEEE\nEXXXX\nEEEEE".to_string();
        assert_eq!(calculate_total_fence_price_with_sides(&input), "236");
    }

    #[test]
    fn test_example_3() {
        let input = "AAAAAA\nAAABBA\nAAABBA\nABBAAA\nABBAAA\nAAAAAA".to_string();
        assert_eq!(calculate_total_fence_price_with_sides(&input), "368");
    }
}
