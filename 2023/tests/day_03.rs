use core::{
    input_reader::{parse_input, parse_response, AOCDay, AOCEdition},
    Input, PuzzleTrait,
};
use y2023::puzzles::d03::Puzzle;

#[test]
fn example() {
    let puzzle = Puzzle {
        input: Input {
            value: parse_input(AOCEdition::Y2023, AOCDay::D03, Some("example".to_string())),
        },
    };

    assert_eq!(puzzle.run(), parse_response(AOCEdition::Y2023, AOCDay::D03, Some("example".to_string())).trim());
}
