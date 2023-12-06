use std::collections::HashMap;

use core::{Input, PuzzleTrait};

pub struct Puzzle {
    pub input: Input,
    pub parse_digits_spelled_out_with_letters: bool,
}

impl PuzzleTrait for Puzzle {
    fn run(&self) -> String {
        let mut nums = vec![
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("0", 0),
        ];
        if self.parse_digits_spelled_out_with_letters {
            let mut spelled_digits = vec![
                ("one", 1),
                ("two", 2),
                ("three", 3),
                ("four", 4),
                ("five", 5),
                ("six", 6),
                ("seven", 7),
                ("eight", 8),
                ("nine", 9),
            ];
            nums.append(&mut spelled_digits);
        }

        let nums_map: HashMap<&str, i32> = HashMap::from_iter(nums);

        let input = &self.input.value;
        let result = input
            .lines()
            .map(|line| {
                return nums_map
                    .iter()
                    .flat_map(|(k, _)| line.match_indices(k))
                    .map(|(k, v)| (k, nums_map.get(v).unwrap()))
                    .collect();
            })
            .map(|match_indices: Vec<(usize, &i32)>| {
                let (_, first) = match_indices.iter().min_by_key(|(index, _)| index).unwrap();
                let (_, last) = match_indices.iter().max_by_key(|(index, _)| index).unwrap();
                return (**first * 10) + **last;
            })
            .sum::<i32>();

        return format!("{}", result);
    }
}
