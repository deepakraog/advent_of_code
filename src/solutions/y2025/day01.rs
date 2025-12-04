pub fn count_zero_visits(input: &str) -> String {
    let mut position = 50; // Dial starts at 50
    let mut count = 0; // Count how many times we visit 0
    
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        // Parse the rotation: first character is direction, rest is distance
        let direction = line.chars().next().unwrap();
        let distance: i32 = line[1..].parse().unwrap();
        
        // Apply rotation
        match direction {
            'L' => {
                // Rotate left (decrease)
                position = (position - distance).rem_euclid(100);
            }
            'R' => {
                // Rotate right (increase)
                position = (position + distance).rem_euclid(100);
            }
            _ => panic!("Invalid direction: {}", direction),
        }
        
        // Check if we're at 0
        if position == 0 {
            count += 1;
        }
    }
    
    count.to_string()
}

pub fn count_zero_visits_during_rotations(input: &str) -> String {
    let mut position = 50i32; // Dial starts at 50
    let mut count = 0u64; // Count how many times we visit 0 (during rotations and at end)
    
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        // Parse the rotation: first character is direction, rest is distance
        let direction = line.chars().next().unwrap();
        let distance: i32 = line[1..].parse().unwrap();
        
        // Count zeros during rotation
        match direction {
            'L' => {
                // Rotate left (decrease)
                // Simulate the rotation step by step to count zeros
                for _ in 0..distance {
                    position = (position - 1).rem_euclid(100);
                    if position == 0 {
                        count += 1;
                    }
                }
            }
            'R' => {
                // Rotate right (increase)
                // We pass through 0 when crossing from 99 to 0
                // Simulate the rotation step by step to count zeros
                for _ in 0..distance {
                    position = (position + 1).rem_euclid(100);
                    if position == 0 {
                        count += 1;
                    }
                }
            }
            _ => panic!("Invalid direction: {}", direction),
        }
    }
    
    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(count_zero_visits(input), "3");
    }

    #[test]
    fn test_example_part2() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(count_zero_visits_during_rotations(input), "6");
    }
}

