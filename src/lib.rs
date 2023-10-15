pub mod grid;
pub mod reader;
pub mod solution;
pub mod solution_set;
pub mod solver;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SolutionError {
    NoSolutionExists,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Colour {
    Black,
    White,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Run {
    pub colour: Colour,
    pub length: usize,
}

impl Run {
    pub fn black(length: usize) -> Self {
        Run { colour: Colour::Black, length }
    }

    pub fn white(length: usize) -> Self {
        Run { colour: Colour::White, length }
    }
}
