# aoc-rs

The Advent of Code with Rust language

## Project layout

```txt
.
├── .gitignore
├── 2023/
│  ├── Cargo.toml
│  ├── src
│  │  ├── bin     // All puzzle solvers (no logic here, one binary per day)
│  │  ├── inputs  // All input data (one mod per day)
│  │  ├── puzzles // All libraries needed to solve the various puzzles (all the logic must be here, one mod per day)
│  │  └── lib.rs
│  └── tests      // Test all puzzles with examples provided by https://adventofcode.com (one file per day)
├── Cargo.lock
├── Cargo.toml
├── core
│  ├── Cargo.toml
│  ├── src        // Basic helpers to structure the project
│  └── tests
├── LICENSE
└── README.md

```
