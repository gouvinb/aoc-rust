use core::{
    input_reader::{parse_input, parse_response, AOCDay, AOCEdition},
    Input, PuzzleTrait,
};
use y2023::puzzles::d02::{PuzzlePart1, PuzzlePart2};

#[test]
fn example_part_1() {
    let puzzle = PuzzlePart1 {
        r_cube_count: 12,
        g_cube_count: 13,
        b_cube_count: 14,

        input: Input {
            value: parse_input(AOCEdition::Y2023, AOCDay::D02, Some("example".to_string())),
        },
    };

    assert_eq!(
        puzzle.run(),
        parse_response(AOCEdition::Y2023, AOCDay::D02, Some("1_example".to_string())).trim()
    );
}

#[test]
fn example_part_2() {
    let puzzle = PuzzlePart2 {
        r_cube_count: 12,
        g_cube_count: 13,
        b_cube_count: 14,

        input: Input {
            value: parse_input(AOCEdition::Y2023, AOCDay::D02, Some("example".to_string())),
        },
    };

    assert_eq!(
        puzzle.run(),
        parse_response(AOCEdition::Y2023, AOCDay::D02, Some("2_example".to_string())).trim()
    );
}
