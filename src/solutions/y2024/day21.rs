use std::collections::HashMap;

type KeypadCache = HashMap<((usize, usize), (usize, usize), usize), usize>;

fn solve_keypad(input: &str, depth: usize) -> usize {
    let numpad = [
        b"789".to_vec(),
        b"456".to_vec(),
        b"123".to_vec(),
        b" 0A".to_vec(),
    ];

    let mut cache: KeypadCache = HashMap::new();

    // Recursive function with memoization to calculate moves
    fn do_move(
        from: (usize, usize),
        to: (usize, usize),
        depth: usize,
        cache: &mut KeypadCache,
    ) -> usize {
        if let Some(&result) = cache.get(&(from, to, depth)) {
            return result;
        }

        let (x1, y1) = from;
        let (x2, y2) = to;

        if depth == 0 {
            return x1.abs_diff(x2) + y1.abs_diff(y2) + 1; // Add 1 for pressing the key
        }

        let mut ans = usize::MAX;

        // Handle moves along both X and Y directions
        if y1 != 0 || (depth == 25 && x2 != 3) || (depth != 25 && x2 != 0) {
            let mut cur = 0;
            let mut pos = (0, 2);

            for _ in x1..x2 {
                cur += do_move(pos, (1, 1), depth - 1, cache);
                pos = (1, 1);
            }
            for _ in x2..x1 {
                cur += do_move(pos, (0, 1), depth - 1, cache);
                pos = (0, 1);
            }
            for _ in y1..y2 {
                cur += do_move(pos, (1, 2), depth - 1, cache);
                pos = (1, 2);
            }
            for _ in y2..y1 {
                cur += do_move(pos, (1, 0), depth - 1, cache);
                pos = (1, 0);
            }
            ans = cur + do_move(pos, (0, 2), depth - 1, cache);
        }

        if y2 != 0 || (depth == 25 && x1 != 3) || (depth != 25 && x1 != 0) {
            let mut cur = 0;
            let mut pos = (0, 2);

            for _ in y1..y2 {
                cur += do_move(pos, (1, 2), depth - 1, cache);
                pos = (1, 2);
            }
            for _ in y2..y1 {
                cur += do_move(pos, (1, 0), depth - 1, cache);
                pos = (1, 0);
            }
            for _ in x1..x2 {
                cur += do_move(pos, (1, 1), depth - 1, cache);
                pos = (1, 1);
            }
            for _ in x2..x1 {
                cur += do_move(pos, (0, 1), depth - 1, cache);
                pos = (0, 1);
            }
            ans = ans.min(cur + do_move(pos, (0, 2), depth - 1, cache));
        }

        cache.insert((from, to, depth), ans);
        ans
    }

    let mut total_cost = 0;

    for line in input.lines() {
        let mut cur_cost = 0;
        let mut current_pos = (3, 2); // Start at 'A'
        let mut num = 0;

        for &key in line.as_bytes() {
            for (i, row) in numpad.iter().enumerate() {
                if let Some(j) = row.iter().position(|&c| c == key) {
                    let move_cost = do_move(current_pos, (i, j), depth, &mut cache);
                    cur_cost += move_cost;
                    current_pos = (i, j);
                    if key.is_ascii_digit() {
                        num = num * 10 + (key - b'0') as usize;
                    }
                    break;
                }
            }
        }

        total_cost += cur_cost * num;
    }

    total_cost
}

pub fn solve_keypad_part1(input: &str) -> String {
    solve_keypad(input, 2).to_string()
}

pub fn solve_keypad_part2(input: &str) -> String {
    solve_keypad(input, 25).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = r"413A
480A
682A
879A
083A";

        assert_eq!(solve_keypad_part1(input), "174242");
        assert_eq!(solve_keypad_part2(input), "220493992841852");
    }
}
