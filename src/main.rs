use aoc::run_solution;
use clap::Parser;

#[derive(Parser)]
struct Args {
    year: u32,
    day: u32,
    part: u32,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let key = (args.year, args.day, args.part);

    let input =
        std::fs::read_to_string(format!("src/input/y{}/day{:02}.txt", args.year, args.day))?;

    println!("{}", run_solution(key, &input));

    Ok(())
}
