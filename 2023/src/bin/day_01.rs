use core::{
    Input,
    input_reader::{AOCDay, AOCEdition, parse_input},
    PuzzleTrait,
};
use y2023::puzzles::d01::Puzzle;

fn main() {
    let puzzle_part_1 = Puzzle {
        parse_digits_spelled_out_with_letters: false,
        input: Input {
            value: parse_input(AOCEdition::Y2023, AOCDay::D01, None)
        },
    };

    println!("puzzle_part_1: {}", puzzle_part_1.run());

    let puzzle_part_2 = Puzzle {
        parse_digits_spelled_out_with_letters: true,
        input: Input {
            value: parse_input(AOCEdition::Y2023, AOCDay::D01, None)
        },
    };

    println!("puzzle_part_2: {}", puzzle_part_2.run());
}
