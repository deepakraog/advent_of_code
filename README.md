Here’s the updated `README.md` with your solution results included:

````markdown
# 🎄 Advent of Code Solutions in Rust 🦀

This repository contains solutions for [Advent of Code](https://adventofcode.com) challenges implemented in Rust. It includes tools and scripts to manage inputs, run solutions, and automate submissions.

Currently, this repository includes solutions for the 2024 Advent of Code puzzles. The long-term goal is to extend this repository to include solutions for all Advent of Code puzzles starting from 2015.

While the existing solutions aim for correctness and clarity, there is room for optimization. Contributions are welcome to refine these solutions, making them more efficient and idiomatic in Rust. Collaboration is encouraged to make this repository a resourceful guide for solving Advent of Code challenges in Rust.

## 🛠️ Getting Started

### 1. Install Dependencies

This project uses [aoc-cli](https://github.com/scarvalhojr/aoc-cli) to manage puzzle inputs and submissions. Install it with:

```bash
cargo install aoc-cli
```
````

### 2. Configure Your Session

Set up your Advent of Code session cookie for authentication. Follow the [aoc-cli setup guide](https://github.com/scarvalhojr/aoc-cli#setup) for details.

---

## 📜 Workflow: From Input to Submission

### Step 1: Fetch Puzzle Input

Use the provided `fetch.sh` script to download the puzzle input and description for a specific day and year:

```bash
./fetch.sh 2024 1  # Fetches input for Day 1 of Year 2024
```

### Step 2: Run the Solution

Run the solution for a specific day, year, and part:

```bash
cargo run -- 2024 1 1  # Runs the solution for Year 2024, Day 1, Part 1
```

### Step 3: Submit the Answer

Use the `solve.sh` script to submit your solution:

```bash
./solve.sh 2024 1 1  # Submits the answer for Part 1 of Day 1, Year 2024
```

### Step 4: Test Your Solution

Run the included unit tests to validate your implementation:

```bash
cargo test
```

---

## 📂 Repository Structure

```
src/
│
├── elves.rs             # Utility file for shared logic or helper functions
│
├── input/               # Directory for puzzle inputs and descriptions
│   └── y2024/           # Inputs and descriptions for Year 2024
│       ├── day01.md     # Puzzle description for Day 1
│       ├── day01.txt    # Puzzle input for Day 1
│       ├── day02.md     # Puzzle description for Day 2
│       ├── day02.txt    # Puzzle input for Day 2
│       ├── day03.md     # Puzzle description for Day 3
│       ├── day03.txt    # Puzzle input for Day 3
│       ├── ......
│
├── lib.rs               # Main library file, includes module imports for solutions
│
├── main.rs              # Binary entry point for the application
│
└── solutions/           # Directory for solutions to puzzles
    └── y2024/           # Solutions for Year 2024
        ├── day01.rs     # Solution implementation for Day 1
        ├── day02.rs     # Solution implementation for Day 2
        ├── day03.rs     # Solution implementation for Day 3
        ├── .......

```

---

## ✨ Example Workflow

### Step 1: Fetch Input

```bash
./fetch.sh 2024 1
```

### Step 2: Run Solution

```bash
cargo run -- 2024 1 1
```

### Step 3: Test Solution

```bash
cargo test
```

### Step 4: Submit Answer

```bash
./solve.sh 2024 1 1
```

---

## 🌟 Contributing

If you'd like to contribute:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-name`).
3. Make your changes and add tests.
4. Submit a pull request!

---

## 🎉 📈 Progress Card

2024 (https://adventofcode.com/2024/leaderboard/private/view/4743704)
![Solution Results](https://github.com/user-attachments/assets/1d4bef48-1171-4e37-b467-d549a861ceee)

---

## 🔗 Resources

- [Advent of Code](https://adventofcode.com)
- [Rust Programming Language](https://www.rust-lang.org)
- [aoc-cli on GitHub](https://github.com/scarvalhojr/aoc-cli)

Happy coding! 🎉

```

```
