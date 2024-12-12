use regex::Regex;

pub fn sum_of_valid_mul_instructions(input: &str) -> String {
    // Regex to match valid `mul(X,Y)` instructions
    let re = Regex::new(r"mul\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*\)").unwrap();

    // Sum up all valid multiplications
    let sum: i64 = re
        .captures_iter(input)
        .map(|caps| {
            let x: i64 = caps[1].parse().unwrap();
            let y: i64 = caps[2].parse().unwrap();
            x * y
        })
        .sum();

    sum.to_string()
}

pub fn sum_of_executable_mul_instructions(input: &str) -> String {
    // Regex patterns for mul, do, and don't instructions
    let control_re = Regex::new(r"\b(?:mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();

    let mut can_execute = true;
    let sum: i64 = control_re
        .captures_iter(input)
        .filter_map(|cap| match cap.get(0).unwrap().as_str() {
            "do()" => {
                can_execute = true;
                None
            }
            "don't()" => {
                can_execute = false;
                None
            }
            _ => {
                if can_execute {
                    let x: i64 = cap[1].parse().unwrap();
                    let y: i64 = cap[2].parse().unwrap();
                    Some(x * y)
                } else {
                    None
                }
            }
        })
        .sum();

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_valid_mul_instructions() {
        let input = "xmul(2,4)&mul[3,7]!@^do_not_mul(5,5)+mul(32,64](mul(11,8)mul(8,5))";
        assert_eq!(sum_of_valid_mul_instructions(input), "161");
    }

    #[test]
    fn test_sum_of_executable_mul_instructions() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(sum_of_executable_mul_instructions(input), "0");
    }
}
