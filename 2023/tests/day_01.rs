use core::{
    input_reader::{parse_input, parse_response, AOCDay, AOCEdition},
    Input, PuzzleTrait,
};
use y2023::puzzles::d01::Puzzle;

#[test]
fn example_part_1() {
    let puzzle = Puzzle {
        parse_digits_spelled_out_with_letters: false,

        input: Input {
            value: parse_input(AOCEdition::Y2023, AOCDay::D01, Some("1_example".to_string())),
        },
    };

    assert_eq!(
        puzzle.run(),
        parse_response(AOCEdition::Y2023, AOCDay::D01, Some("1_example".to_string())).trim()
    );
}

#[test]
fn example_part_2() {
    let puzzle_part_1 = Puzzle {
        parse_digits_spelled_out_with_letters: true,

        input: Input {
            value: parse_input(AOCEdition::Y2023, AOCDay::D01, Some("1_example".to_string())),
        },
    };

    assert_eq!(
        puzzle_part_1.run(),
        parse_response(AOCEdition::Y2023, AOCDay::D01, Some("1_example".to_string())).trim()
    );

    let puzzle_part_2 = Puzzle {
        parse_digits_spelled_out_with_letters: true,
        input: Input {
            value: parse_input(AOCEdition::Y2023, AOCDay::D01, Some("2_example".to_string())),
        },
    };

    assert_eq!(
        puzzle_part_2.run(),
        parse_response(AOCEdition::Y2023, AOCDay::D01, Some("2_example".to_string())).trim()
    );
}
