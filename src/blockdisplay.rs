use std::fmt::{Display, Write};

use super::Colour;
use super::grid::{TrialColour, TrialGrid};

pub struct BlockDisplay<'a> {
    grid: &'a TrialGrid,
}

impl<'a> BlockDisplay<'a> {
    pub fn new(grid: &'a TrialGrid) -> Self {
        Self { grid }
    }
}

impl Display for BlockDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TrialColour::{Unknown, Any, Maybe, Only};
        use Colour::{Black, White};
        for row in 0..self.grid.height() {
            for cell in self.grid.row(row) {
                match cell {
                    Unknown | Any => f.write_char(' ')?,
                    // Maybe(Black) | Only(Black) => f.write_char('▅')?,
                    Maybe(Black) | Only(Black) => f.write_char('█')?,
                    Maybe(White) | Only(White) => f.write_char('.')?,
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}
