pub mod input_reader;

pub struct Input {
    pub value: String,
}

pub trait PuzzleTrait {
    fn run(&self) -> String;
}
