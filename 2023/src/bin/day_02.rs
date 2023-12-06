use core::{
    input_reader::{parse_input, AOCDay, AOCEdition},
    Input, PuzzleTrait,
};
use y2023::puzzles::d02::{PuzzlePart1, PuzzlePart2};

fn main() {
    let puzzle_part_1 = PuzzlePart1 {
        r_cube_count: 12,
        g_cube_count: 13,
        b_cube_count: 14,

        input: Input {
            value: parse_input(AOCEdition::Y2023, AOCDay::D02, None),
        },
    };

    println!("puzzle_part_1: {}", puzzle_part_1.run());

    let puzzle_part_2 = PuzzlePart2 {
        r_cube_count: 12,
        g_cube_count: 13,
        b_cube_count: 14,

        input: Input {
            value: parse_input(AOCEdition::Y2023, AOCDay::D02, None),
        },
    };

    println!("puzzle_part_2: {}", puzzle_part_2.run());
}
