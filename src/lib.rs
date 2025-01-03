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
        pub mod day16;
        pub mod day17;
        pub mod day18;
        pub mod day19;
        pub mod day20;
        pub mod day21;
        pub mod day22;
        pub mod day23;
        pub mod day24;
        pub mod day25;
        pub mod helpers;
    }
}

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
            (13, 2) => y2024::day13::solve_claw_contraption_part2(input),
            (14, 1) => y2024::day14::solve_part1(input),
            (14, 2) => y2024::day14::solve_part2(input),
            (15, 1) => y2024::day15::solve_part1(input),
            (15, 2) => y2024::day15::solve_part2(input),
            (16, 1) => y2024::day16::solve_part1(input),
            (16, 2) => y2024::day16::solve_part2(input),
            (17, 1) => y2024::day17::solve_part1(input),
            (17, 2) => y2024::day17::solve_part2(input),
            (18, 1) => y2024::day18::solve_part1(input),
            (18, 2) => y2024::day18::solve_part2(input),
            (19, 1) => y2024::day19::solve_part1(input),
            (19, 2) => y2024::day19::solve_part2(input),
            (20, 1) => y2024::day20::solve_part1(input),
            (20, 2) => y2024::day20::solve_part2(input),
            (21, 1) => y2024::day21::solve_part1(input),
            (21, 2) => y2024::day21::solve_part2(input),
            (22, 1) => y2024::day22::solve_part1(input),
            (22, 2) => y2024::day22::solve_part2(input),
            (23, 1) => y2024::day23::solve_part1(input),
            (23, 2) => y2024::day23::solve_part2(input),
            (24, 1) => y2024::day24::solve_part1(input),
            (24, 2) => y2024::day24::solve_part2(input),
            (25, 1) => y2024::day25::valid_pairs(input),
            _ => "Invalid option".to_string(),
        },
        _ => "Not implemented yet".to_string(),
    }
}
