use std::iter::Map;
use std::str::Lines;

use core::{
    Input,
    PuzzleTrait,
};

pub struct PuzzlePart1 {
    pub input: Input,

    pub r_cube_count: i32,
    pub g_cube_count: i32,
    pub b_cube_count: i32,
}

impl PuzzleTrait for PuzzlePart1 {
    fn run(&self) -> String {
        let result: i32 = self.input.value
            .parse_all_games()
            .filter(|(_, (r, g, b))| { return r <= &self.r_cube_count && g <= &self.g_cube_count && b <= &self.b_cube_count; })
            .map(|(id, _)| { return id; })
            .sum();

        return format!("{}", result);
    }
}

pub struct PuzzlePart2 {
    pub input: Input,

    pub r_cube_count: i32,
    pub g_cube_count: i32,
    pub b_cube_count: i32,
}

impl PuzzleTrait for PuzzlePart2 {
    fn run(&self) -> String {
        let result: i32 = self.input.value
            .parse_all_games()
            .map(|(_, (r, g, b))| { return r * g * b; })
            .sum();

        return format!("{}", result);
    }
}

trait GameHelper {
    fn parse_all_games(&self) -> Map<Lines, fn(&str) -> (i32, (i32, i32, i32))>;
}

impl GameHelper for String {
    fn parse_all_games(&self) -> Map<Lines, fn(&str) -> (i32, (i32, i32, i32))> {
        return self.lines()
            .map(|line| {
                let (game_name, draws_raw) = line
                    .split_once(": ")
                    .expect(format!("Format not supported (line: {}", line).as_str());

                let id = game_name
                    .trim_start_matches("Game ")
                    .parse::<i32>()
                    .expect(format!("Id not found with game name (Game name: {})", game_name).as_str());

                let draws: Vec<(String, i32)> = draws_raw
                    .split("; ")
                    .flat_map(|draw| {
                        return draw
                            .split(", ")
                            .map(|sub_draw| {
                                let (color_count_raw, color_name) = sub_draw
                                    .split_once(" ")
                                    .expect(format!("Cannot find color and count with `{}`", sub_draw).as_str());
                                let color_count = color_count_raw
                                    .parse::<i32>()
                                    .expect(format!("Cannot count color {}. (Sub draw: {})", color_name, sub_draw).as_str());
                                return (color_name.to_string(), color_count);
                            });
                    })
                    .collect();

                let (max_red, max_green, max_blue) = (
                    draws.max_count_of("red".to_string()),
                    draws.max_count_of("green".to_string()),
                    draws.max_count_of("blue".to_string()),
                );
                return (id, (*max_red, *max_green, *max_blue));
            });
    }
}

trait DrawsHelper {
    fn max_count_of(&self, color_name: String) -> &i32;
}

impl DrawsHelper for Vec<(String, i32)> {
    fn max_count_of(&self, color_name: String) -> &i32 {
        return self
            .iter()
            .filter(|(color, _)| { return *color == color_name; })
            .map(|(_, count)| { return count; })
            .max()
            .expect(format!("Cannot return max of {:?}", self).as_str());
    }
}
