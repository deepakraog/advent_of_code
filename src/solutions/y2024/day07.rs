pub fn sum_valid_equations(input: &str) -> String {
    let mut total_sum = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        let target = parts[0].trim().parse::<i64>().unwrap();
        let ns: Vec<i64> = parts[1]
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        if is_valid_equation(target, ns.clone(), false) {
            total_sum += target;
        }
    }

    total_sum.to_string()
}

pub fn sum_valid_equations_with_concat(input: &str) -> String {
    let mut total_sum = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        let target = parts[0].trim().parse::<i64>().unwrap();
        let ns: Vec<i64> = parts[1]
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        if is_valid_equation(target, ns.clone(), true) {
            total_sum += target;
        }
    }

    total_sum.to_string()
}

fn is_valid_equation(target: i64, ns: Vec<i64>, allow_concat: bool) -> bool {
    fn helper(target: i64, ns: &[i64], allow_concat: bool) -> bool {
        if ns.len() == 1 {
            return ns[0] == target;
        }

        let rest = ns[2..].to_vec();
        if helper(
            target,
            &[&[ns[0] + ns[1]], &rest[..]].concat(),
            allow_concat,
        ) {
            return true;
        }
        if helper(
            target,
            &[&[ns[0] * ns[1]], &rest[..]].concat(),
            allow_concat,
        ) {
            return true;
        }
        if allow_concat {
            // Concatenate numbers as strings, then parse back to integer
            let concatenated = format!("{}{}", ns[0], ns[1])
                .parse::<i64>()
                .unwrap_or(i64::MAX);
            if helper(target, &[&[concatenated], &rest[..]].concat(), allow_concat) {
                return true;
            }
        }
        false
    }

    helper(target, &ns, allow_concat)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_valid_equations() {
        let input = "190: 10 19\n3267: 81 40 27\n83: 17 5";
        assert_eq!(sum_valid_equations(input), "3457");
    }

    #[test]
    fn test_sum_valid_equations_with_concat() {
        let input = "156: 15 6\n7290: 6 8 6 15\n192: 17 8 14";
        assert_eq!(sum_valid_equations_with_concat(input), "7638");
    }
}
