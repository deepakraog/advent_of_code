pub mod solutions {
    pub mod y2024 {
        pub mod day01;
        pub mod day02;
        pub mod day03;
        pub mod day04;
        pub mod day05;
        pub mod day06;
        pub mod day07;
        pub mod day08;
        pub mod day09;
        pub mod day10;
        pub mod day11;
        pub mod day12;
        pub mod day13;
        pub mod day14;
        pub mod day15;
    }
}

/// elves are helpful
pub mod elves;

use solutions::*;

pub fn run_solution((year, day, part): (u32, u32, u32), input: &str) -> String {
    match year {
        2024 => match (day, part) {
            (1, 1) => y2024::day01::calculate_total_distance(input),
            (1, 2) => y2024::day01::calculate_similarity_score(input),
            (2, 1) => y2024::day02::count_safe_reports(input),
            (2, 2) => y2024::day02::count_safe_reports_with_dampener(input),
            (3, 1) => y2024::day03::sum_of_valid_mul_instructions(input),
            (3, 2) => y2024::day03::sum_of_executable_mul_instructions(input),
            (4, 1) => y2024::day04::count_xmas(input),
            (4, 2) => y2024::day04::count_patterns(input),
            (5, 1) => y2024::day05::sum_middle_pages_correctly_ordered(input),
            (5, 2) => y2024::day05::sum_middle_pages_after_fixing_order(input),
            (6, 1) => y2024::day06::count_distinct_positions(input),
            (6, 2) => y2024::day06::count_trapping_obstruction_positions(input),
            (7, 1) => y2024::day07::sum_valid_equations(input),
            (7, 2) => y2024::day07::sum_valid_equations_with_concat(input),
            (8, 1) => y2024::day08::calculate_antinodes(input),
            (8, 2) => y2024::day08::calculate_with_harmonics(input),
            (9, 1) => y2024::day09::calculate_checksum(input),
            (9, 2) => y2024::day09::calculate_checksum_fragmentation(input),
            (10, 1) => y2024::day10::sum_trailhead_scores(input),
            (10, 2) => y2024::day10::sum_trailhead_ratings(input),
            (11, 1) => y2024::day11::count_stones_after_blinks(input, 25),
            (11, 2) => y2024::day11::count_stones_after_blinks(input, 75),
            (12, 1) => y2024::day12::calculate_total_fence_price(input),
            (12, 2) => y2024::day12::calculate_total_fence_price_with_sides(input),
            (13, 1) => y2024::day13::solve_claw_contraption_part1(input),
            (13, 2) => y2024::day13::solve_claw_contraption_part2(input), // Not Working
            (14, 1) => y2024::day14::solve_part1(input),
            (14, 2) => y2024::day14::solve_part2(input), // Not Working
            (15, 1) => y2024::day15::solve_part1(input),
            (15, 2) => y2024::day15::solve_part2(input),
            _ => "Not implemented yet".to_string(),
        },
        _ => "Not implemented yet".to_string(),
    }
}
