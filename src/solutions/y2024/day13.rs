use std::collections::HashMap;

/// Solves the problem for a single machine.
fn solve(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64, part2: bool) -> i64 {
    let p2_offset = if part2 { 1_000_000_000_000 } else { 0 };
    let target_x = px + p2_offset;
    let target_y = py + p2_offset;

    let mut best: Option<(i64, i64, i64, i64)> = None;

    // Try combinations of t1 and t2
    for t1 in 0..200 {
        for t2 in 0..200 {
            let cost = 3 * t1 + t2;
            let dx = ax * t1 + bx * t2;
            let dy = ay * t1 + by * t2;

            if dx == dy && dx > 0 && (best.is_none() || dx / cost < best.unwrap().0) {
                best = Some((dx / cost, t1, t2, cost));
            }
        }
    }

    if best.is_none() {
        return 0;
    }

    let (_, t1, t2, cost) = best.unwrap();

    // Memoized recursive function to calculate minimum cost to reach (x, y)
    let mut dp = HashMap::new();
    fn f(
        x: i64,
        y: i64,
        ax: i64,
        ay: i64,
        bx: i64,
        by: i64,
        dp: &mut HashMap<(i64, i64), i64>,
    ) -> i64 {
        if let Some(&cached) = dp.get(&(x, y)) {
            return cached;
        }
        if x == 0 && y == 0 {
            return 0;
        }
        if x < 0 || y < 0 {
            return i64::MAX / 2;
        }

        let cost_a = 3 + f(x - ax, y - ay, ax, ay, bx, by, dp);
        let cost_b = 1 + f(x - bx, y - by, ax, ay, bx, by, dp);

        let result = cost_a.min(cost_b);
        dp.insert((x, y), result);
        result
    }

    let amt = (p2_offset - 40_000) / (ax * t1 + bx * t2);
    let remaining_cost = f(
        target_x - amt * (ax * t1 + bx * t2),
        target_y - amt * (ay * t1 + by * t2),
        ax,
        ay,
        bx,
        by,
        &mut dp,
    );

    if remaining_cost < i64::MAX / 2 {
        remaining_cost + amt * cost
    } else {
        0
    }
}

/// Parses the input and solves for Part 1 and Part 2.
pub fn solution(input: &str) -> (i64, i64) {
    let machines: Vec<&str> = input.split("\n\n").collect();
    let mut part1 = 0;
    let mut part2 = 0;

    for machine in machines {
        let lines: Vec<&str> = machine.lines().collect();
        if lines.len() < 3 {
            continue;
        }

        // Parse input values
        let parse_number = |s: &str| -> i64 {
            s.split(['+', '=', ','])
                .filter_map(|x| x.trim().parse::<i64>().ok())
                .next()
                .unwrap_or(0)
        };

        let ax = parse_number(lines[0].split_whitespace().nth(2).unwrap_or(""));
        let ay = parse_number(lines[0].split_whitespace().nth(3).unwrap_or(""));
        let bx = parse_number(lines[1].split_whitespace().nth(2).unwrap_or(""));
        let by = parse_number(lines[1].split_whitespace().nth(3).unwrap_or(""));
        let px = parse_number(lines[2].split_whitespace().nth(1).unwrap_or(""));
        let py = parse_number(lines[2].split_whitespace().nth(2).unwrap_or(""));

        part1 += solve(ax, ay, bx, by, px, py, false);
        part2 += solve(ax, ay, bx, by, px, py, true);
    }

    (part1, part2)
}

pub fn solve_claw_contraption_part1(input: &str) -> String {
    solution(input).0.to_string()
}

pub fn solve_claw_contraption_part2(input: &str) -> String {
    solution(input).1.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = r"Button A: X+49, Y+27
Button B: X+14, Y+7
Prize: X=756, Y=504

Button A: X+21, Y+8
Button B: X+12, Y+4
Prize: X=504, Y=216";

        assert_eq!(solve_claw_contraption_part1(input), "0");
        assert_eq!(solve_claw_contraption_part2(input), "0");
    }
}
