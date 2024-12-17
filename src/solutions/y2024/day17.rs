use std::collections::HashMap;

/// Executes the Chronospatial Computer program and returns the output as a comma-separated string.
pub fn execute_program(input: &str) -> String {
    // Parse the input into registers and program
    let (mut registers, program) = parse_input(input);

    // Instruction pointer starts at 0
    let mut ip = 0;
    let mut output = Vec::new();

    // Execute the program
    while ip < program.len() {
        let opcode = program[ip];
        let operand = program[ip + 1];
        match opcode {
            0 => {
                // adv: Divide A by 2^operand (combo)
                let divisor = combo_value(operand, &registers).pow(2);
                registers.insert('A', registers[&'A'] / divisor);
            }
            1 => {
                // bxl: Bitwise XOR B with literal operand
                let literal = operand;
                registers.insert('B', registers[&'B'] ^ literal as i64);
            }
            2 => {
                // bst: Set B to combo operand % 8
                registers.insert('B', combo_value(operand, &registers) % 8);
            }
            3 => {
                // jnz: Jump to operand if A != 0
                if registers[&'A'] != 0 {
                    ip = operand;
                    continue;
                }
            }
            4 => {
                // bxc: Bitwise XOR B with C
                registers.insert('B', registers[&'B'] ^ registers[&'C']);
            }
            5 => {
                // out: Output combo operand % 8
                output.push((combo_value(operand, &registers) % 8).to_string());
            }
            6 => {
                // bdv: Like adv but stores result in B
                let divisor = combo_value(operand, &registers).pow(2);
                registers.insert('B', registers[&'A'] / divisor);
            }
            7 => {
                // cdv: Like adv but stores result in C
                let divisor = combo_value(operand, &registers).pow(2);
                registers.insert('C', registers[&'A'] / divisor);
            }
            _ => panic!("Invalid opcode: {}", opcode),
        }
        ip += 2; // Advance instruction pointer by 2
    }

    output.join(",")
}

/// Parses the input and extracts the registers and program.
fn parse_input(input: &str) -> (HashMap<char, i64>, Vec<usize>) {
    let mut registers = HashMap::new();
    let mut program = Vec::new();

    let lines: Vec<&str> = input.lines().collect();
    for line in &lines {
        if line.starts_with("Register A:") {
            registers.insert(
                'A',
                line.split_whitespace().nth(2).unwrap().parse().unwrap(),
            );
        } else if line.starts_with("Register B:") {
            registers.insert(
                'B',
                line.split_whitespace().nth(2).unwrap().parse().unwrap(),
            );
        } else if line.starts_with("Register C:") {
            registers.insert(
                'C',
                line.split_whitespace().nth(2).unwrap().parse().unwrap(),
            );
        } else if line.starts_with("Program:") {
            program = line
                .split_whitespace()
                .skip(1)
                .flat_map(|s| s.split(',').map(|x| x.parse::<usize>().unwrap()))
                .collect();
        }
    }

    (registers, program)
}

/// Computes the value of a combo operand.
fn combo_value(operand: usize, registers: &HashMap<char, i64>) -> i64 {
    match operand {
        0..=3 => operand as i64,
        4 => registers[&'A'],
        5 => registers[&'B'],
        6 => registers[&'C'],
        _ => panic!("Invalid combo operand: {}", operand),
    }
}
