use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

type ButtonSequences = HashMap<(char, char), Vec<String>>;

/// Compute sequences for moving between buttons on the keypad.
fn compute_sequences(keypad: &[&str]) -> ButtonSequences {
    let mut positions = HashMap::new();
    let size_x = i32::try_from(keypad[0].len()).unwrap();
    let size_y = i32::try_from(keypad.len()).unwrap();

    // Store positions of buttons
    for (y, row) in keypad.iter().enumerate() {
        for (x, button) in row.chars().enumerate() {
            if button != ' ' {
                positions.insert(button, (x as i32, y as i32));
            }
        }
    }

    let mut sequences: ButtonSequences = HashMap::new();
    for &from_button in positions.keys() {
        for &to_button in positions.keys() {
            if from_button == to_button {
                sequences.insert((from_button, to_button), vec!["A".to_string()]);
                continue;
            }

            let mut possibilities = Vec::new();
            let mut queue = VecDeque::new();
            let mut shortest = usize::MAX;
            let mut visited = HashMap::new();

            queue.push_front((positions[&from_button], String::new()));
            visited.insert(positions[&from_button], 0);

            while let Some(((x, y), moves)) = queue.pop_back() {
                if (x, y) == positions[&to_button] {
                    if moves.len() < shortest {
                        shortest = moves.len();
                        possibilities.clear();
                    }

                    if moves.len() == shortest {
                        possibilities.push(format!("{moves}A"));
                    }
                    continue;
                }

                for (nx, ny, nm) in [
                    (x - 1, y, '<'),
                    (x + 1, y, '>'),
                    (x, y - 1, '^'),
                    (x, y + 1, 'v'),
                ] {
                    if nx < 0 || nx >= size_x || ny < 0 || ny >= size_y {
                        continue;
                    }

                    let button = keypad[ny as usize].chars().nth(nx as usize).unwrap();
                    if button == ' ' {
                        continue;
                    }

                    if *visited.get(&(nx, ny)).unwrap_or(&usize::MAX) >= moves.len() {
                        queue.push_front(((nx, ny), format!("{moves}{nm}")));
                        visited.insert((nx, ny), moves.len());
                    }
                }
            }

            sequences.insert((from_button, to_button), possibilities);
        }
    }

    sequences
}

struct Solver {
    numerical_sequences: ButtonSequences,
    directional_sequences: ButtonSequences,
}

impl Solver {
    fn new() -> Self {
        let numerical_keypad = ["789", "456", "123", " 0A"];
        let directional_keypad = [" ^A", "<v>"];
        Self {
            numerical_sequences: compute_sequences(&numerical_keypad),
            directional_sequences: compute_sequences(&directional_keypad),
        }
    }

    fn find_code_seqs(&self, code: &str) -> Vec<String> {
        let mut sequences = Vec::new();
        for i in 0..code.len() {
            let button_from = if i == 0 {
                'A'
            } else {
                code.chars().nth(i - 1).unwrap()
            };
            let button_to = code.chars().nth(i).unwrap();
            sequences.push(self.numerical_sequences[&(button_from, button_to)].clone());
        }
        sequences
            .iter()
            .multi_cartesian_product()
            .map(|steps| steps.iter().join(""))
            .collect()
    }

    fn compute_seq_length(
        &self,
        target_seq: &str,
        robots: u32,
        cache: &mut HashMap<(String, u32), u64>,
    ) -> u64 {
        if let Some(&cached) = cache.get(&(target_seq.to_string(), robots)) {
            return cached;
        }

        if robots <= 1 {
            let result: u64 = (0..target_seq.len())
                .map(|i| {
                    let from = if i == 0 {
                        'A'
                    } else {
                        target_seq.chars().nth(i - 1).unwrap()
                    };
                    let to = target_seq.chars().nth(i).unwrap();
                    self.directional_sequences[&(from, to)][0].len() as u64
                })
                .sum();
            cache.insert((target_seq.to_string(), robots), result);
            return result;
        }

        let result: u64 = (0..target_seq.len())
            .map(|i| {
                let from = if i == 0 {
                    'A'
                } else {
                    target_seq.chars().nth(i - 1).unwrap()
                };
                let to = target_seq.chars().nth(i).unwrap();
                self.directional_sequences[&(from, to)]
                    .iter()
                    .map(|seq| self.compute_seq_length(seq, robots - 1, cache))
                    .min()
                    .unwrap()
            })
            .sum();

        cache.insert((target_seq.to_string(), robots), result);
        result
    }

    fn complexity(&self, code: &str, robots: u32) -> u64 {
        let seqs = self.find_code_seqs(code);
        let mut cache = HashMap::new();
        let min_length = seqs
            .iter()
            .map(|seq| self.compute_seq_length(seq, robots, &mut cache))
            .min()
            .unwrap();
        let num_code = code
            .chars()
            .map_while(|c| c.to_digit(10))
            .fold(0, |acc, d| acc * 10 + d);
        min_length * u64::from(num_code)
    }
}

pub fn solve_part1(input: &str) -> String {
    let codes: Vec<String> = input.lines().map(|line| line.to_string()).collect();
    let solver = Solver::new();
    codes
        .iter()
        .map(|code| solver.complexity(code, 2))
        .sum::<u64>()
        .to_string()
}

pub fn solve_part2(input: &str) -> String {
    let codes: Vec<String> = input.lines().map(|line| line.to_string()).collect();
    let solver = Solver::new();
    codes
        .iter()
        .map(|code| solver.complexity(code, 25))
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "029A\n980A\n";
        assert_eq!(solve_part1(input), "60772");
        assert_eq!(solve_part2(input), "73176637651790");
    }
}
