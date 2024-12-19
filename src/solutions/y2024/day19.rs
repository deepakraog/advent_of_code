use std::collections::{HashMap, HashSet};

/// Parses the input and returns the towel patterns and desired designs.
fn parse_input(input: &str) -> (HashSet<String>, Vec<String>) {
    let mut sections = input.split("\n\n");
    let towel_patterns = sections
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect();
    let designs = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect();
    (towel_patterns, designs)
}

/// Counts the number of ways to form a design using the available towel patterns.
fn count_design_ways(
    design: &str,
    towel_patterns: &HashSet<String>,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if design.is_empty() {
        return 1; // Base case: An empty design has 1 way to be formed
    }

    if let Some(&cached) = memo.get(design) {
        return cached; // Return cached result if available
    }

    let mut count = 0;
    for i in 1..=design.len() {
        if towel_patterns.contains(&design[..i]) {
            count += count_design_ways(&design[i..], towel_patterns, memo);
        }
    }

    memo.insert(design.to_string(), count); // Cache result
    count
}

/// Solves Part 1.
pub fn solve_part1(input: &str) -> String {
    let (towel_patterns, designs) = parse_input(input);

    let count = designs
        .iter()
        .filter(|design| {
            let mut memo = HashMap::new();
            count_design_ways(design, &towel_patterns, &mut memo) > 0
        })
        .count();

    count.to_string()
}

/// Solves Part 2.
pub fn solve_part2(input: &str) -> String {
    let (towel_patterns, designs) = parse_input(input);

    let total_ways: usize = designs
        .iter()
        .map(|design| {
            let mut memo = HashMap::new();
            count_design_ways(design, &towel_patterns, &mut memo)
        })
        .sum();

    total_ways.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case_part1() {
        let input = r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!(solve_part1(input), "6");
    }

    #[test]
    fn test_example_case_part2() {
        let input = r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!(solve_part2(input), "16");
    }
}
