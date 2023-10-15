use std::fmt::{Display, Write};
use std::ops::{Index, IndexMut};

use super::Colour;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TrialColour {
    Unknown,
    Any,
    Maybe(Colour),
    Only(Colour),
}

pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Colour>,
}

impl Grid {
    pub fn new(width: usize, height: usize, colour: Colour) -> Self {
        let mut cells = Vec::new();
        cells.resize(width * height, colour);
        Self { width, height, cells }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Colour;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        &self.cells[row * self.width + col]        
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, col) = index;
        &mut self.cells[row * self.width + col]
    }
}

pub struct TrialGrid {
    width: usize,
    height: usize,
    cells: Vec<TrialColour>,
}

impl TrialGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells = Vec::new();
        cells.resize(width * height, TrialColour::Unknown);
        Self { width, height, cells }
    }
}

impl Index<(usize, usize)> for TrialGrid {
    type Output = TrialColour;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        &self.cells[row * self.width + col]
    }
}

impl IndexMut<(usize, usize)> for TrialGrid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, col) = index;
        &mut self.cells[row * self.width + col]
    }
}

impl Display for TrialGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TrialColour::{Unknown, Any, Maybe, Only};
        let width_with_spacers = self.width + (self.width + 4) / 5 - 1;
        let separator_row = "-".repeat(width_with_spacers);
        for row in 0..self.height {
            if row > 0 && (row % 5) == 0 {
                f.write_str(&separator_row)?;
                f.write_char('\n')?;
            }
            for col in 0..self.width {
                if col > 0 && (col % 5) == 0 {
                    f.write_char('|')?;
                }
                match self[(row, col)] {
                    Unknown | Any => f.write_char('.')?,
                    Maybe(Colour::Black) | Only(Colour::Black) => f.write_char('#')?,
                    Maybe(Colour::White) | Only(Colour::White) => f.write_char('X')?,
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}
