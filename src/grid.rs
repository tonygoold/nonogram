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

impl TrialColour {
    pub fn is_only(&self) -> bool {
        match *self {
            TrialColour::Only(_) => true,
            _ => false,
        }
    }

    pub fn and_trial(&self, rhs: &TrialColour) -> TrialColour {
        use TrialColour::{Unknown, Any, Maybe, Only};
        match (*self, *rhs) {
            (Unknown, _) => *rhs,
            (_, Unknown) => *self,
            (Only(_), _) => *self,
            (_, Only(_)) => *rhs,
            (Maybe(c1), Maybe(c2)) => if c1 == c2 { *self } else { Any },
            (Any, _) => *self,
            (_, Any) => *rhs,
        }
    }

    pub fn and(&self, rhs: &Colour) -> TrialColour {
        use TrialColour::{Unknown, Any, Maybe, Only};
        match (*self, *rhs) {
            (Unknown, colour) => Maybe(colour),
            (Any, _) => Any,
            (Maybe(c1), c2) => if c1 == c2 { *self } else { Any },
            (Only(_), _) => *self,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
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

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn solved(&self) -> bool {
        self.cells.iter().all(TrialColour::is_only)
    }

    pub fn convert_maybe_to_only(&mut self) {
        for cell in self.cells.iter_mut() {
            if let TrialColour::Maybe(colour) = cell {
                *cell = TrialColour::Only(*colour);
            }
        }
    }

    pub fn row(&self, index: usize) -> Vec<TrialColour> {
        (0..self.width).map(|col| self[(index, col)]).collect()
    }

    pub fn col(&self, index: usize) -> Vec<TrialColour> {
        (0..self.height).map(|row| self[(row, index)]).collect()
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
                    Unknown | Any => f.write_char('?')?,
                    Maybe(_) => f.write_char('\'')?,
                    Only(Colour::Black) => f.write_char('#')?,
                    Only(Colour::White) => f.write_char('.')?,
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}
