use std::cmp::min;

/// Represents the configuration of a claw machine.
struct ClawMachine {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    prize_x: i64,
    prize_y: i64,
}

/// Parses the input and returns a vector of `ClawMachine` configurations.
fn parse_input(input: &str, add_offset: bool) -> Vec<ClawMachine> {
    let mut machines = Vec::new();

    for chunk in input.split("\n\n") {
        let mut a_x = 0;
        let mut a_y = 0;
        let mut b_x = 0;
        let mut b_y = 0;
        let mut prize_x = 0;
        let mut prize_y = 0;

        for line in chunk.lines() {
            if line.starts_with("Button A:") {
                let parts: Vec<&str> = line.split(',').collect();
                a_x = parse_coordinate(parts[0]).unwrap_or(0);
                a_y = parse_coordinate(parts[1]).unwrap_or(0);
            } else if line.starts_with("Button B:") {
                let parts: Vec<&str> = line.split(',').collect();
                b_x = parse_coordinate(parts[0]).unwrap_or(0);
                b_y = parse_coordinate(parts[1]).unwrap_or(0);
            } else if line.starts_with("Prize:") {
                let parts: Vec<&str> = line.split(',').collect();
                prize_x = parse_coordinate(parts[0]).unwrap_or(0);
                prize_y = parse_coordinate(parts[1]).unwrap_or(0);
                if add_offset {
                    let offset = 10_000_000_000_000;
                    prize_x += offset;
                    prize_y += offset;
                }
            }
        }

        machines.push(ClawMachine {
            a_x,
            a_y,
            b_x,
            b_y,
            prize_x,
            prize_y,
        });
    }

    machines
}

/// Parses a coordinate string (e.g., "X+94" or "X=8400") and returns the numeric value.
fn parse_coordinate(coord: &str) -> Result<i64, std::num::ParseIntError> {
    let value = coord.split(|c| c == '+' || c == '=').nth(1).unwrap_or("0");
    value.parse()
}

/// Solves Part 1 of the claw contraption problem.
pub fn solve_claw_contraption_part1(input: &str) -> String {
    let machines = parse_input(input, false);
    let mut total_tokens = 0;

    for machine in machines {
        if let Some(tokens) = find_minimum_tokens(
            machine.a_x,
            machine.a_y,
            machine.b_x,
            machine.b_y,
            machine.prize_x,
            machine.prize_y,
        ) {
            total_tokens += tokens;
        }
    }

    total_tokens.to_string()
}

/// Solves Part 2 of the claw contraption problem.
pub fn solve_claw_contraption_part2(input: &str) -> String {
    let machines = parse_input(input, true);
    let mut total_tokens = 0;

    for machine in machines {
        if let Some(tokens) = find_minimum_tokens_extended(
            machine.a_x,
            machine.a_y,
            machine.b_x,
            machine.b_y,
            machine.prize_x,
            machine.prize_y,
        ) {
            total_tokens += tokens;
        }
    }

    total_tokens.to_string()
}

/// Finds the minimum tokens required to align the claw for one machine with a 100 press limit.
fn find_minimum_tokens(
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    prize_x: i64,
    prize_y: i64,
) -> Option<i64> {
    let max_presses = 100;
    let mut min_tokens = None;

    for a_presses in 0..=max_presses {
        for b_presses in 0..=max_presses {
            let x_move = a_presses * a_x + b_presses * b_x;
            let y_move = a_presses * a_y + b_presses * b_y;

            if x_move == prize_x && y_move == prize_y {
                let tokens = a_presses * 3 + b_presses * 1; // A costs 3 tokens, B costs 1 token
                min_tokens = Some(min(min_tokens.unwrap_or(tokens), tokens));
            }
        }
    }

    min_tokens
}

/// Finds the minimum tokens required to align the claw for one machine without press limits.
fn find_minimum_tokens_extended(
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    prize_x: i64,
    prize_y: i64,
) -> Option<i64> {
    let (gcd_x, x_a, x_b) = extended_gcd(a_x, b_x);
    let (gcd_y, y_a, y_b) = extended_gcd(a_y, b_y);

    if prize_x % gcd_x != 0 || prize_y % gcd_y != 0 {
        return None;
    }

    let x_scale = prize_x / gcd_x;
    let y_scale = prize_y / gcd_y;

    let a_presses_x = x_a * x_scale;
    let b_presses_x = x_b * x_scale;

    let a_presses_y = y_a * y_scale;
    let b_presses_y = y_b * y_scale;

    if a_presses_x >= 0 && b_presses_x >= 0 && a_presses_y >= 0 && b_presses_y >= 0 {
        let total_tokens =
            (a_presses_x * 3 + b_presses_x * 1) + (a_presses_y * 3 + b_presses_y * 1);
        return Some(total_tokens);
    }

    None
}

/// Extended Euclidean Algorithm to solve `a * x + b * y = gcd(a, b)`.
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x, y) = extended_gcd(b, a % b);
        (gcd, y, x - (a / b) * y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example_input() {
        let input = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

        assert_eq!(solve_claw_contraption_part1(input), "480");
    }

    #[test]
    fn test_part2_example_input() {
        let input = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

        assert_eq!(solve_claw_contraption_part2(input), "100000000480");
    }
}
