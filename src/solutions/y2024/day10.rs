use std::collections::{HashMap, HashSet};

// Type alias for the return type of `parse_topographic_map`
type ParsedTopography = (Vec<Vec<u32>>, usize, usize, Vec<Vec<(usize, usize)>>);

/// Parses the input and extracts the grid, dimensions, and node locations by height.
pub fn parse_topographic_map(input: &str) -> ParsedTopography {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect();

    let n = grid.len();
    let m = grid[0].len();
    let mut loc_by_height: Vec<Vec<(usize, usize)>> = vec![vec![]; 10];

    for (i, row) in grid.iter().enumerate() {
        for (j, &height) in row.iter().enumerate() {
            loc_by_height[height as usize].push((i, j));
        }
    }

    (grid, n, m, loc_by_height)
}

/// Calculates the sum of trailhead scores.
pub fn sum_trailhead_scores(input: &str) -> String {
    let (grid, n, m, loc_by_height) = parse_topographic_map(input);
    let mut reachable_locs: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();

    // Initialize reachable locations for height 9
    for &(x, y) in &loc_by_height[9] {
        reachable_locs.insert((x, y), HashSet::from([(x, y)]));
    }

    let mut total_score = 0;
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    // Process heights from 8 down to 0
    for h in (0..9).rev() {
        for &(x, y) in &loc_by_height[h] {
            let mut current_reachable = HashSet::new();
            for &(dx, dy) in &directions {
                let newx = x as isize + dx;
                let newy = y as isize + dy;

                if newx >= 0
                    && newx < n as isize
                    && newy >= 0
                    && newy < m as isize
                    && grid[newx as usize][newy as usize] == h as u32 + 1
                {
                    if let Some(neighbors) = reachable_locs.get(&(newx as usize, newy as usize)) {
                        current_reachable.extend(neighbors.iter().cloned());
                    }
                }
            }
            reachable_locs.insert((x, y), current_reachable);

            if h == 0 {
                total_score += reachable_locs[&(x, y)].len();
            }
        }
    }

    total_score.to_string()
}

/// Calculates the sum of trailhead ratings.
pub fn sum_trailhead_ratings(input: &str) -> String {
    let (grid, n, m, loc_by_height) = parse_topographic_map(input);
    let mut reachable_locs: HashMap<(usize, usize), usize> = HashMap::new();

    // Initialize reachable counts for height 9
    for &(x, y) in &loc_by_height[9] {
        reachable_locs.insert((x, y), 1);
    }

    let mut total_rating = 0;
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    // Process heights from 8 down to 0
    for h in (0..9).rev() {
        for &(x, y) in &loc_by_height[h] {
            let mut current_count = 0;
            for &(dx, dy) in &directions {
                let newx = x as isize + dx;
                let newy = y as isize + dy;

                if newx >= 0
                    && newx < n as isize
                    && newy >= 0
                    && newy < m as isize
                    && grid[newx as usize][newy as usize] == h as u32 + 1
                {
                    if let Some(&neighbor_count) =
                        reachable_locs.get(&(newx as usize, newy as usize))
                    {
                        current_count += neighbor_count;
                    }
                }
            }
            reachable_locs.insert((x, y), current_count);

            if h == 0 {
                total_rating += current_count;
            }
        }
    }

    total_rating.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_trailhead_scores() {
        let input = "0123\n1234\n8765\n9876";
        assert_eq!(sum_trailhead_scores(input), "1");
    }

    #[test]
    fn test_sum_trailhead_ratings() {
        let input = "0123\n1234\n8765\n9876";
        assert_eq!(sum_trailhead_ratings(input), "16");
    }

    #[test]
    fn test_larger_example_scores() {
        let input =
            "89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732";
        assert_eq!(sum_trailhead_scores(input), "36");
    }

    #[test]
    fn test_larger_example_ratings() {
        let input =
            "89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732";
        assert_eq!(sum_trailhead_ratings(input), "81");
    }
}
