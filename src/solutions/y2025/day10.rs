use std::collections::{HashSet, VecDeque};

struct Machine {
    target: Vec<bool>,        // Target light configuration
    buttons: Vec<Vec<usize>>, // Each button toggles which lights
}

fn parse_machine(line: &str) -> Machine {
    // Parse format: [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    let line = line.trim();

    // Extract indicator lights diagram [.##.]
    let start = line.find('[').unwrap() + 1;
    let end = line.find(']').unwrap();
    let diagram = &line[start..end];
    let target: Vec<bool> = diagram.chars().map(|c| c == '#').collect();

    // Extract buttons between ] and {
    let buttons_start = end + 1;
    let buttons_end = line.find('{').unwrap();
    let buttons_str = &line[buttons_start..buttons_end].trim();

    let mut buttons = Vec::new();
    let mut i = 0;
    while i < buttons_str.len() {
        if buttons_str.chars().nth(i) == Some('(') {
            let mut j = i + 1;
            while j < buttons_str.len() && buttons_str.chars().nth(j) != Some(')') {
                j += 1;
            }
            let button_str = &buttons_str[i + 1..j];
            let toggles: Vec<usize> = button_str
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            buttons.push(toggles);
            i = j + 1;
        } else {
            i += 1;
        }
    }

    Machine { target, buttons }
}

fn solve_machine_min_presses(machine: &Machine) -> usize {
    let n = machine.target.len();

    // Use BFS to find minimum button presses
    // State: bitmask representing current light state
    let target_state: u64 = machine
        .target
        .iter()
        .enumerate()
        .map(|(i, &on)| if on { 1u64 << i } else { 0 })
        .sum();

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((0u64, 0usize)); // (state, num_presses)
    visited.insert(0);

    while let Some((state, presses)) = queue.pop_front() {
        if state == target_state {
            return presses;
        }

        for button in &machine.buttons {
            let mut new_state = state;
            for &light_idx in button {
                if light_idx < n {
                    new_state ^= 1u64 << light_idx;
                }
            }

            if !visited.contains(&new_state) {
                visited.insert(new_state);
                queue.push_back((new_state, presses + 1));
            }
        }
    }

    // Should not reach here if solution exists
    0
}

pub fn sum_minimum_button_presses(input: &str) -> String {
    let mut total = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let machine = parse_machine(line);
        let min_presses = solve_machine_min_presses(&machine);
        total += min_presses;
    }

    total.to_string()
}

// Parse machine for Part 2: returns (buttons as bitmasks, joltages)
// buttons[i] is a bitmask where bit j is set if button i increments counter j
fn parse_machine_part2_optimized(line: &str) -> (Vec<usize>, Vec<i32>) {
    let line = line.trim();

    // Extract buttons between ] and {
    let end = line.find(']').unwrap();
    let buttons_start = end + 1;
    let buttons_end = line.find('{').unwrap();
    let buttons_str = &line[buttons_start..buttons_end].trim();

    let mut buttons = Vec::new();
    let mut i = 0;
    while i < buttons_str.len() {
        if buttons_str.chars().nth(i) == Some('(') {
            let mut j = i + 1;
            while j < buttons_str.len() && buttons_str.chars().nth(j) != Some(')') {
                j += 1;
            }
            let button_str = &buttons_str[i + 1..j];
            // Convert to bitmask: button[i] has bit j set if it increments counter j
            let button_mask: usize = button_str
                .split(',')
                .map(|s| s.trim().parse::<usize>().unwrap())
                .fold(0, |acc, idx| acc | (1 << idx));
            buttons.push(button_mask);
            i = j + 1;
        } else {
            i += 1;
        }
    }

    // Extract joltage requirements {3,5,4,7}
    let joltage_start = line.find('{').unwrap() + 1;
    let joltage_end = line.find('}').unwrap();
    let joltage_str = &line[joltage_start..joltage_end];
    let joltages: Vec<i32> = joltage_str
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    (buttons, joltages)
}

// Solve Integer Linear Programming: minimize sum(x) for A*x == b with x >= 0 and integer x
fn solve_ilp(a: &[Vec<i32>], b: &[i32]) -> Vec<i32> {
    let m = a.len();
    let n = a[0].len();

    // Form augmented matrix tableau [A | b]
    let mut t = vec![vec![0.0f32; n + 1]; m];
    for i in 0..m {
        for j in 0..n {
            t[i][j] = a[i][j] as f32;
        }
        t[i][n] = b[i] as f32;
    }

    // Track which variables are free and which are bound
    let mut inonz = vec![-1i32; m]; // First non-zero index in each row after reduction
    let mut ifree = Vec::new();
    let mut h = 0;
    let mut k = 0;

    // Gaussian elimination to reduced row echelon form
    while h < m && k < n {
        // Find pivot (row with largest absolute value in column k)
        let mut imax = h;
        let mut amax = t[h][k].abs();
        #[allow(clippy::needless_range_loop)]
        for i in (h + 1)..m {
            if t[i][k].abs() > amax {
                amax = t[i][k].abs();
                imax = i;
            }
        }

        if amax < 0.0001f32 {
            // Column k is free
            ifree.push(k);
            k += 1;
        } else {
            // Column k is bound (pivot found)
            inonz[h] = k as i32;

            // Swap rows h and imax
            if h != imax {
                t.swap(h, imax);
            }

            // Reduce to RREF
            let pivot_val = t[h][k];
            for i in 0..m {
                if i == h {
                    continue;
                }
                let f = t[i][k] / pivot_val;
                t[i][k] = 0.0f32;
                #[allow(clippy::needless_range_loop)]
                for j in (k + 1)..=n {
                    t[i][j] -= t[h][j] * f;
                }
            }

            h += 1;
            k += 1;
        }
    }

    // Get remaining free variables
    for k in m..n {
        if !ifree.contains(&k) && !inonz.contains(&(k as i32)) {
            ifree.push(k);
        }
    }

    // Compute upper bounds for each variable
    let mut imaxes = vec![0i32; n];
    for j in 0..n {
        for i in 0..m {
            if a[i][j] != 0 {
                imaxes[j] = imaxes[j].max(b[i]);
            }
        }
    }

    // Find the maximum index of non-free variables (for back-substitution)
    let mut i0 = -1i32;
    for i in (0..m).rev() {
        if inonz[i] >= 0 {
            i0 = i as i32;
            break;
        }
    }

    // Search over free variable combinations
    let mut xopt = vec![0.0f32; n];
    let mut fopt = 1e20f32;
    let mut combos = vec![0i32; ifree.len()];
    let imaxes_free: Vec<i32> = ifree.iter().map(|&idx| imaxes[idx]).collect();

    loop {
        let mut sumx = combos.iter().sum::<i32>() as f32;

        if sumx < fopt {
            let mut x = vec![0.0f32; n];
            // Set free variables
            for (idx, &val) in ifree.iter().zip(combos.iter()) {
                x[*idx] = val as f32;
            }

            let mut is_valid = true;

            // Back-substitute to solve for bound variables
            for i in (0..=i0 as usize).rev() {
                if inonz[i] < 0 {
                    continue;
                }
                let k = inonz[i] as usize;

                x[k] = t[i][n];
                // Subtract contributions from free variables
                for (free_idx, &free_val) in ifree.iter().zip(combos.iter()) {
                    x[k] -= t[i][*free_idx] * (free_val as f32);
                }
                x[k] /= t[i][k];

                // Check validity
                if x[k] < -0.0001f32 {
                    is_valid = false;
                    break;
                }
                if !is_integer(x[k]) {
                    is_valid = false;
                    break;
                }
                sumx += x[k];
                if sumx >= fopt {
                    is_valid = false;
                    break;
                }
            }

            if is_valid {
                fopt = sumx;
                xopt = x;
            }
        }

        // Move to next combination
        if !next_combo(&mut combos, &imaxes_free) {
            break;
        }
    }

    // Round solution to integers
    let mut iopt = vec![0i32; n];
    for i in 0..n {
        iopt[i] = xopt[i] as i32;
        if (iopt[i] as f32) < xopt[i] - 0.5f32 {
            iopt[i] += 1;
        }
    }

    iopt
}

fn is_integer(x: f32) -> bool {
    let frac = x.abs() % 1.0f32;
    !(0.0001f32..=0.9999f32).contains(&frac)
}

fn next_combo(c: &mut [i32], n: &[i32]) -> bool {
    // Increment combination (like bignum += 1 with mixed radix)
    if c.is_empty() {
        return false;
    }

    let mut i = 0;
    while i < c.len() && c[i] == n[i] - 1 {
        c[i] = 0;
        i += 1;
    }

    if i == c.len() {
        return false;
    }

    c[i] += 1;
    true
}

fn configure_joltages(buttons: &[usize], joltages: &[i32]) -> i32 {
    let num_buttons = buttons.len();
    let num_jolts = joltages.len();

    // Build matrix A: A[i][j] = 1 if button j increments counter i, else 0
    let mut a = vec![vec![0i32; num_buttons]; num_jolts];
    for (i, _joltage) in joltages.iter().enumerate() {
        for j in 0..num_buttons {
            if buttons[j] & (1 << i) != 0 {
                a[i][j] = 1;
            }
        }
    }

    // Solve ILP: minimize sum(x) where A*x == joltages, x >= 0, x integer
    let xopt = solve_ilp(&a, joltages);
    xopt.iter().sum()
}

pub fn sum_minimum_button_presses_part2(input: &str) -> String {
    let mut total = 0i32;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (buttons, joltages) = parse_machine_part2_optimized(line);
        let min_presses = configure_joltages(&buttons, &joltages);
        total += min_presses;
    }

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(sum_minimum_button_presses(input), "7");
    }

    #[test]
    fn test_single_machine() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        assert_eq!(sum_minimum_button_presses(input), "2");
    }

    #[test]
    fn test_example_part2() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(sum_minimum_button_presses_part2(input), "33");
    }

    #[test]
    fn test_single_machine_part2() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        assert_eq!(sum_minimum_button_presses_part2(input), "10");
    }
}
