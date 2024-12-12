use std::collections::{HashMap, HashSet};

pub fn parse_input(
    input: &str,
) -> (
    Vec<Vec<char>>,
    usize,
    usize,
    HashMap<char, Vec<(usize, usize)>>,
) {
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let n = grid.len();
    let m = grid[0].len();
    let mut nodes = HashMap::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell != '.' {
                nodes.entry(cell).or_insert_with(Vec::new).push((i, j));
            }
        }
    }

    (grid, n, m, nodes)
}

pub fn calculate_antinodes(input: &str) -> String {
    let (_, n, m, nodes) = parse_input(input);
    let mut antinodes = HashSet::new();

    for (_, node_list) in nodes {
        let l = node_list.len();
        for i in 0..l {
            for j in 0..i {
                let (x1, y1) = node_list[i];
                let (x2, y2) = node_list[j];

                // Calculate forward antinode
                let newx = x2 as isize + (x2 as isize - x1 as isize);
                let newy = y2 as isize + (y2 as isize - y1 as isize);
                if newx >= 0 && newx < n as isize && newy >= 0 && newy < m as isize {
                    antinodes.insert((newx as usize, newy as usize));
                }

                // Calculate reverse antinode
                let newx = x1 as isize + (x1 as isize - x2 as isize);
                let newy = y1 as isize + (y1 as isize - y2 as isize);
                if newx >= 0 && newx < n as isize && newy >= 0 && newy < m as isize {
                    antinodes.insert((newx as usize, newy as usize));
                }
            }
        }
    }

    antinodes.len().to_string()
}

pub fn calculate_with_harmonics(input: &str) -> String {
    let (_, n, m, nodes) = parse_input(input);
    let mut antinodes = HashSet::new();

    for (_, node_list) in nodes {
        let l = node_list.len();
        for i in 0..l {
            antinodes.insert(node_list[i]); // Each antenna is now an antinode
            for j in 0..i {
                let (x1, y1) = node_list[i];
                let (x2, y2) = node_list[j];
                let dx = x2 as isize - x1 as isize;
                let dy = y2 as isize - y1 as isize;

                // Forward direction
                let mut newx = x2 as isize + dx;
                let mut newy = y2 as isize + dy;
                while newx >= 0 && newx < n as isize && newy >= 0 && newy < m as isize {
                    antinodes.insert((newx as usize, newy as usize));
                    newx += dx;
                    newy += dy;
                }

                // Reverse direction
                let mut newx = x1 as isize - dx;
                let mut newy = y1 as isize - dy;
                while newx >= 0 && newx < n as isize && newy >= 0 && newy < m as isize {
                    antinodes.insert((newx as usize, newy as usize));
                    newx -= dx;
                    newy -= dy;
                }
            }
        }
    }

    antinodes.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_antinodes() {
        let input = "............
                     ........0...
                     .....0......
                     .......0....
                     ....0.......
                     ......A.....
                     ............
                     ............
                     ........A...
                     .........A..
                     ............
                     ............";
        assert_eq!(calculate_antinodes(input), "144");
    }

    #[test]
    fn test_calculate_with_harmonics() {
        let input = "............
                     ........0...
                     .....0......
                     .......0....
                     ....0.......
                     ......A.....
                     ............
                     ............
                     ........A...
                     .........A..
                     ............
                     ............";
        assert_eq!(calculate_with_harmonics(input), "250");
    }
}
