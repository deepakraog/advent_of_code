use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
enum Role {
    CarryOut,     // the Cout wire
    IntXorXor,    // intermediate wire between the two XOR gates
    ABAndGate,    // intermediate wires between AB and the (bottom) AND gate
    AndGateWires, // wiring of the AND gates
    SumOut,       // the S wire
}

#[derive(Eq, PartialEq, Hash, Clone)]
enum Operation {
    And,
    Or,
    Xor,
}

impl Operation {
    fn from(s: &str) -> Self {
        match s {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            _ => panic!("unknown operation: {}", s),
        }
    }

    const fn eval(&self, a: u8, b: u8) -> u8 {
        match self {
            Self::And => a & b,
            Self::Or => a | b,
            Self::Xor => a ^ b,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Gate {
    a: String,     // input wire
    b: String,     // input wire
    op: Operation, // type of gate
    r: String,     // output wire
}

fn is_role(set: &HashSet<Role>, role: &Role) -> bool {
    set.len() == 1 && set.iter().next().unwrap() == role
}

fn is_roles(set: &HashSet<Role>, role1: &Role, role2: &Role) -> bool {
    set.len() == 2 && {
        let roles: Vec<_> = set.iter().collect();
        (roles[0] == role1 && roles[1] == role2) || (roles[0] == role2 && roles[1] == role1)
    }
}

struct Puzzle {
    wires: HashMap<String, u8>,
    gates: Vec<Gate>,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            wires: HashMap::new(),
            gates: Vec::new(),
        }
    }

    /// Parse the input and configure the puzzle.
    fn configure(&mut self, input: &str) {
        for line in input.lines() {
            if line.contains(": ") {
                let (wire, value) = line.split_once(": ").unwrap();
                self.wires.insert(wire.to_string(), value.parse().unwrap());
            }
            if line.contains(" -> ") {
                let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
                let gate = Gate {
                    a: parts[0].to_string(),
                    op: Operation::from(parts[1]),
                    b: parts[2].to_string(),
                    r: parts[4].to_string(),
                };
                self.gates.push(gate);
            }
        }
    }

    /// Solve part one.
    fn solve_part1(&self) -> String {
        let mut waiting_gates = self.gates.iter().collect::<Vec<_>>();
        let mut wires = self.wires.clone();

        while !waiting_gates.is_empty() {
            let mut next_waiting = Vec::new();

            for gate in &waiting_gates {
                if let Some(&a) = wires.get(&gate.a) {
                    if let Some(&b) = wires.get(&gate.b) {
                        let r = gate.op.eval(a, b);
                        wires.insert(gate.r.clone(), r);
                        continue;
                    }
                }
                next_waiting.push(*gate);
            }

            waiting_gates = next_waiting;
        }

        let result = wires
            .iter()
            .filter(|(r, &v)| r.starts_with('z') && v == 1)
            .fold(0_u64, |acc, (r, _)| {
                acc | (1 << r[1..].parse::<u64>().unwrap())
            });

        result.to_string()
    }

    /// Solve part two.
    fn solve_part2(&self) -> String {
        let mut input_types: HashMap<&str, HashSet<Role>> = HashMap::new();
        let mut result_types: HashMap<&str, HashSet<Role>> = HashMap::new();

        for gate in &self.gates {
            let mut add_result_role =
                |r: &Role| result_types.entry(&gate.r).or_default().insert(r.clone());

            if (gate.a.starts_with('x') && gate.b.starts_with('y'))
                || (gate.a.starts_with('y') && gate.b.starts_with('x'))
            {
                add_result_role(match gate.op {
                    Operation::Xor => &Role::IntXorXor,
                    Operation::And => &Role::ABAndGate,
                    Operation::Or => &Role::CarryOut,
                });
            } else {
                let role = match gate.op {
                    Operation::Xor => &Role::SumOut,
                    Operation::And => &Role::AndGateWires,
                    Operation::Or => &Role::CarryOut,
                };

                input_types.entry(&gate.a).or_default().insert(role.clone());
                input_types.entry(&gate.b).or_default().insert(role.clone());
                add_result_role(role);
            }
        }

        let last_z_wire = result_types
            .keys()
            .filter(|wire| wire.starts_with('z'))
            .max()
            .unwrap();

        let mut bad_wires: Vec<&str> = Vec::new();

        for wire in result_types.keys() {
            let inp = &input_types.entry(wire).or_default();
            let res = &result_types[wire];

            if wire == last_z_wire && is_role(res, &Role::CarryOut) {
                continue;
            }

            if inp.is_empty() && wire.starts_with('z') && is_role(res, &Role::SumOut) {
                continue;
            }

            if is_role(inp, &Role::CarryOut)
                && (is_role(res, &Role::AndGateWires) || is_role(res, &Role::ABAndGate))
            {
                continue;
            }

            if is_roles(inp, &Role::SumOut, &Role::AndGateWires)
                && (is_role(res, &Role::CarryOut) || is_role(res, &Role::IntXorXor))
            {
                continue;
            }

            bad_wires.push(wire);
        }

        bad_wires.sort_unstable();
        bad_wires.join(",")
    }
}

/// Exposed function for part one.
pub fn solve_part1(input: &str) -> String {
    let mut puzzle = Puzzle::new();
    puzzle.configure(input);
    puzzle.solve_part1()
}

/// Exposed function for part two.
pub fn solve_part2(input: &str) -> String {
    let mut puzzle = Puzzle::new();
    puzzle.configure(input);
    puzzle.solve_part2()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = r"x00: 1
x01: 0
x02: 0
x03: 1
x04: 1
y00: 1
y01: 1
y02: 1
y03: 1
y04: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

        assert_eq!(solve_part1(input), "7");
    }

    #[test]
    fn test_part2_example() {
        let input = r"x00: 1
x01: 0
x02: 0
x03: 1
x04: 1
y00: 1
y01: 1
y02: 1
y03: 1
y04: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

        assert_eq!(solve_part2(input), "z00,z01");
    }
}
