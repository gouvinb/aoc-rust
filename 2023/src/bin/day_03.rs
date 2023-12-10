use core::{
    input_reader::{parse_input, AOCDay, AOCEdition},
    Input, PuzzleTrait,
};
use y2023::puzzles::d03::Puzzle;

fn main() {
    let puzzle_part_1 = Puzzle {
        input: Input {
            value: parse_input(AOCEdition::Y2023, AOCDay::D03, None),
        },
    };

    println!("puzzle_part_1: {}", puzzle_part_1.run());
}
