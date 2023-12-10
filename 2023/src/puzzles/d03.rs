use std::cmp::{max, min};

#[allow(unused_imports)]
use core::{Input, PuzzleTrait, scoped_fn::ScopedFn};

pub struct Puzzle {
    pub input: Input,
}

impl PuzzleTrait for Puzzle {
    fn run(&self) -> String {
        let input_lines = self.input.value.lines();

        let grid = input_lines.clone().map(|line| line.chars().collect()).collect::<Grid>();

        let points = grid
            .clone()
            .iter()
            .enumerate()
            .flat_map(|(row, line)| {
                let mut parts_in_line = vec![];
                let mut position = 0;
                while position <= line.len() {
                    let longest_number = (1..(line.len() - position + 1))
                        .map_while(|offset| {
                            let substr = &line[position..(position + offset)];
                            if !substr.iter().all(|c| c.is_ascii_digit()) {
                                return None;
                            }
                            substr.last()
                        })
                        .map(|char| char.to_string())
                        .collect::<String>();
                    if !longest_number.is_empty() {
                        parts_in_line.push(Number::new(longest_number.clone(), position, row));
                    }
                    position += max(longest_number.len(), 1);
                }
                parts_in_line.into_iter()
            })
            .collect::<Vec<Number>>();


        let result: usize = points
            .iter()
            .filter(|point| point.is_engine_number(&grid))
            .map(|point| {
                point
                    .number
                    .parse::<usize>()
                    .unwrap_or_else(|_| panic!("Point.id ({}) must be a usize.", point.number))
            })
            // .also_once(|res| {
            //     res.clone().for_each(|it|
            //         println!("{:?}", it)
            //     );
            // })
            // .clone()
            .sum();

        format!("{}", result)
    }
}

type Grid = Vec<Vec<char>>;

#[derive(Debug)]
struct Number {
    pub number: String,
    pub x: usize,
    pub y: usize,
}

impl Number {
    fn new(number: String, x: usize, y: usize) -> Number {
        Number {
            number,
            x,
            y,
        }
    }

    fn is_engine_number(&self, grid: &Grid) -> bool {
        let height = grid.len();
        let width = grid[0].len();

        let y_range: Vec<usize> = (self.y.saturating_sub(1)..=min(self.y.saturating_add(1), height - 1)).collect();
        let x_range: Vec<usize> = (self.x.saturating_sub(1)..=min(self.x + self.number.len(), width - 1)).collect();

        return !&y_range
            .iter()
            .map(|&y| x_range.iter().map(move |&x| grid[y][x].to_string()).collect::<Vec<_>>().join(""))
            .collect::<Vec<_>>()
            .join("\n")
            // .also_once(|map| {
            //     println!("{}\n{}\n\n", "#".repeat(80), map);
            // })
            .replace(['\n', '.'], "")
            .replace(&self.number, "")
            // .also_mut(|res| {
            //     println!("id {} => symbole == {}", self.number, res);
            // })
            .is_empty();
    }
}
