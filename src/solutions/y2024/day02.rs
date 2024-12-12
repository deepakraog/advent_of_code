pub fn count_safe_reports(input: &str) -> String {
    let reports = parse_input(input);

    // Helper function to check if a single report is safe
    fn is_safe(report: &[i32]) -> bool {
        if report.len() < 2 {
            return false; // A report must have at least two levels
        }

        let mut direction = None; // None, Some(true) = increasing, Some(false) = decreasing

        for window in report.windows(2) {
            let diff = window[1] - window[0];

            if diff.abs() < 1 || diff.abs() > 3 {
                return false; // Difference must be between 1 and 3
            }

            if diff > 0 {
                if direction == Some(false) {
                    return false; // Mixed direction
                }
                direction = Some(true); // Increasing
            } else if diff < 0 {
                if direction == Some(true) {
                    return false; // Mixed direction
                }
                direction = Some(false); // Decreasing
            }
        }

        true
    }

    // Count the number of safe reports
    let count = reports.iter().filter(|report| is_safe(report)).count();
    count.to_string()
}

pub fn count_safe_reports_with_dampener(input: &str) -> String {
    let reports = parse_input(input);

    // Helper function to check if a single report is safe
    fn is_safe(report: &[i32]) -> bool {
        if report.len() < 2 {
            return false; // A report must have at least two levels
        }

        let mut direction = None; // None, Some(true) = increasing, Some(false) = decreasing

        for window in report.windows(2) {
            let diff = window[1] - window[0];

            if diff.abs() < 1 || diff.abs() > 3 {
                return false; // Difference must be between 1 and 3
            }

            if diff > 0 {
                if direction == Some(false) {
                    return false; // Mixed direction
                }
                direction = Some(true); // Increasing
            } else if diff < 0 {
                if direction == Some(true) {
                    return false; // Mixed direction
                }
                direction = Some(false); // Decreasing
            }
        }

        true
    }

    // Helper function to check if a report becomes safe by removing a single level
    fn is_safe_with_dampener(report: &[i32]) -> bool {
        for i in 0..report.len() {
            let mut modified_report = report.to_vec();
            modified_report.remove(i); // Remove one level at index `i`
            if is_safe(&modified_report) {
                return true; // Report becomes safe
            }
        }
        false
    }

    // Count the number of safe reports, considering the Problem Dampener
    let count = reports
        .iter()
        .filter(|report| is_safe(report) || is_safe_with_dampener(report))
        .count();

    count.to_string()
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_safe_reports() {
        let input = "1 3 5 7\n2 4 6 8\n1 2 3 5\n1 3 4 8";
        assert_eq!(count_safe_reports(input), "3");
    }

    #[test]
    fn test_count_safe_reports_with_dampener() {
        let input = "1 3 5 7\n2 4 6 8\n1 2 3 5\n1 3 4 8";
        assert_eq!(count_safe_reports_with_dampener(input), "4");
    }
}
