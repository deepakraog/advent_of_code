use std::str::FromStr;

#[derive(Debug)]
struct Button {
    x: i64,
    y: i64,
    cost: i64,
}

#[derive(Debug)]
struct ArcadeMachine {
    a: Button,
    b: Button,
    target_x: i64,
    target_y: i64,
}

impl ArcadeMachine {
    fn new_from_strs(
        a_x: &str,
        a_y: &str,
        b_x: &str,
        b_y: &str,
        target_x: &str,
        target_y: &str,
    ) -> Result<ArcadeMachine, anyhow::Error> {
        let a_x = a_x.parse()?;
        let a_y = a_y.parse()?;
        let b_x = b_x.parse()?;
        let b_y = b_y.parse()?;
        let target_x = target_x.parse()?;
        let target_y = target_y.parse()?;
        let a = Button {
            x: a_x,
            y: a_y,
            cost: 3,
        };
        let b = Button {
            x: b_x,
            y: b_y,
            cost: 1,
        };
        Ok(ArcadeMachine {
            a,
            b,
            target_x,
            target_y,
        })
    }

    fn min_tokens(&self, offset: i64) -> Option<i64> {
        let target_x = self.target_x.checked_add(offset)?;
        let target_y = self.target_y.checked_add(offset)?;

        // Solve the linear Diophantine equation
        let b_multiplied = self
            .a
            .y
            .checked_mul(target_x)?
            .checked_sub(self.a.x.checked_mul(target_y)?)?;
        let b_divisor = self
            .a
            .y
            .checked_mul(self.b.x)?
            .checked_sub(self.a.x.checked_mul(self.b.y)?)?;

        if b_divisor == 0 {
            return None; // Prevent division by zero
        }

        if b_multiplied % b_divisor != 0 {
            return None; // No solution
        }

        let b = b_multiplied / b_divisor;

        let a_multiplied = target_x.checked_sub(b.checked_mul(self.b.x)?)?;
        if self.a.x == 0 {
            return None; // Prevent division by zero
        }

        if a_multiplied % self.a.x != 0 {
            return None; // No solution
        }

        let a = a_multiplied / self.a.x;

        if a < 0 || b < 0 {
            return None; // Non-negative solutions only
        }

        Some(a.checked_mul(self.a.cost)? + b.checked_mul(self.b.cost)?)
    }
}

#[derive(Debug)]
struct Arcade {
    machines: Vec<ArcadeMachine>,
}

impl FromStr for Arcade {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let machines = input
            .split("\n\n")
            .filter_map(|machine| {
                let lines: Vec<&str> = machine.lines().collect();
                if lines.len() < 3 {
                    return None;
                }

                let parse_number = |s: &str| -> i64 {
                    s.split(['+', '=', ','].as_ref())
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

                ArcadeMachine::new_from_strs(
                    &ax.to_string(),
                    &ay.to_string(),
                    &bx.to_string(),
                    &by.to_string(),
                    &px.to_string(),
                    &py.to_string(),
                )
                .ok()
            })
            .collect();

        Ok(Arcade { machines })
    }
}

impl Arcade {
    fn solve_claw_contraption_part1(&self) -> String {
        self.machines
            .iter()
            .filter_map(|m| m.min_tokens(0))
            .sum::<i64>()
            .to_string()
    }

    fn solve_claw_contraption_part2(&self) -> String {
        self.machines
            .iter()
            .filter_map(|m| m.min_tokens(10_000_000_000_000))
            .sum::<i64>()
            .to_string()
    }
}

pub fn solve_claw_contraption_part1(input: &str) -> String {
    let arcade = Arcade::from_str(input).expect("Failed to parse input");
    arcade.solve_claw_contraption_part1()
}

pub fn solve_claw_contraption_part2(input: &str) -> String {
    let arcade = Arcade::from_str(input).expect("Failed to parse input");
    arcade.solve_claw_contraption_part2()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176";

        let part1_result = solve_claw_contraption_part1(input);
        let part2_result = solve_claw_contraption_part2(input);

        assert_eq!(part1_result, "280");
        assert_eq!(part2_result, "459236326669");
    }
}
