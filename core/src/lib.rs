pub mod input_reader;
pub mod scoped_fn;

pub struct Input {
    pub value: String,
}

pub trait PuzzleTrait {
    fn run(&self) -> String;
}
