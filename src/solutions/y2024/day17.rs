/// Represents the Chronospatial Computer.
pub struct Puzzle {
    reg_a: u32,
    reg_b: u32,
    reg_c: u32,
    program: Vec<u32>,
}

impl Puzzle {
    /// Create a new Puzzle instance.
    pub const fn new() -> Self {
        Self {
            reg_a: 0,
            reg_b: 0,
            reg_c: 0,
            program: Vec::new(),
        }
    }

    /// Parses the input into registers and program.
    pub fn configure(&mut self, input: &str) {
        for line in input.lines() {
            if let Some(v) = line.strip_prefix("Register A: ") {
                self.reg_a = v.parse().unwrap();
            } else if let Some(v) = line.strip_prefix("Register B: ") {
                self.reg_b = v.parse().unwrap();
            } else if let Some(v) = line.strip_prefix("Register C: ") {
                self.reg_c = v.parse().unwrap();
            } else if let Some(v) = line.strip_prefix("Program: ") {
                self.program = v.split(',').filter_map(|i| i.trim().parse().ok()).collect();
            }
        }
    }

    /// Runs the program with given initial register values.
    fn run(&self, mut a: u32, mut b: u32, mut c: u32) -> Vec<u32> {
        let mut ip = 0;
        let mut output = Vec::new();

        while ip < self.program.len() - 1 {
            let opcode = self.program[ip];
            let literal = self.program[ip + 1];

            let combo = || match literal {
                0..=3 => literal,
                4 => a,
                5 => b,
                6 => c,
                _ => panic!("Invalid combo operand"),
            };

            match opcode {
                0 => a >>= literal,   // adv
                1 => b ^= literal,    // bxl
                2 => b = combo() % 8, // bst
                3 => {
                    if a != 0 {
                        ip = literal as usize;
                        continue;
                    }
                } // jnz
                4 => b ^= c,          // bxc
                5 => output.push(combo() % 8), // out
                6 => b = a >> combo(), // bdv
                7 => c = a >> combo(), // cdv
                _ => panic!("Unknown opcode: {}", opcode),
            }

            ip += 2;
        }

        output
    }

    /// Finds the lowest initial value of A that makes the program output itself.
    fn quine(&self, a: u64, i: usize, xor1: u64, xor2: u64) -> u64 {
        let target = self.program[i] as u64;
        let start_octal = u64::from(i == self.program.len() - 1);

        for octal in start_octal..8 {
            let new_a = (a * 8) | octal;
            let mut b = octal ^ xor1;
            let c = new_a >> b;
            b ^= xor2;
            b ^= c;

            if b % 8 == target {
                if i == 0 {
                    return new_a;
                }
                let new_a = self.quine(new_a, i - 1, xor1, xor2);
                if new_a != u64::MAX {
                    return new_a;
                }
            }
        }
        u64::MAX
    }

    /// Solve part one.
    pub fn part1(&self) -> String {
        let output = self.run(self.reg_a, self.reg_b, self.reg_c);
        output
            .iter()
            .map(u32::to_string)
            .collect::<Vec<String>>()
            .join(",")
    }

    /// Solve part two.
    pub fn part2(&self) -> String {
        let xors = self
            .program
            .chunks(2)
            .filter(|instr| instr[0] == 1)
            .map(|instr| instr[1])
            .collect::<Vec<_>>();

        if xors.len() != 2 {
            return "No solution".to_string();
        }

        let xor1 = xors[0] as u64;
        let xor2 = xors[1] as u64;

        let result = self.quine(0, self.program.len() - 1, xor1, xor2);
        if result == u64::MAX {
            "No solution".to_string()
        } else {
            result.to_string()
        }
    }
}

impl Default for Puzzle {
    fn default() -> Self {
        Self::new()
    }
}

/// Solves part one of the puzzle.
pub fn solve_part1(input: &str) -> String {
    let mut puzzle = Puzzle::new();
    puzzle.configure(input);
    puzzle.part1()
}

/// Solves part two of the puzzle.
pub fn solve_part2(input: &str) -> String {
    let mut puzzle = Puzzle::new();
    puzzle.configure(input);
    puzzle.part2()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

        assert_eq!(solve_part1(input), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part2() {
        let input = r"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

        assert_eq!(solve_part2(input), "No solution");
    }
}
