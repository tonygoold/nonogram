use super::SolutionError;
use super::grid::{TrialColour, TrialGrid};
use super::solution_set::SolutionSet;

pub struct Solver {
    width: usize,
    height: usize,
    row_solution_sets: Vec<SolutionSet>,
    col_solution_sets: Vec<SolutionSet>,
}

impl Solver {
    pub fn new(width: usize, height: usize, rows_groups: &Vec<Vec<usize>>, cols_groups: &Vec<Vec<usize>>) -> Result<Self, SolutionError> {
        let mut row_solution_sets = Vec::new();
        let mut col_solution_sets = Vec::new();
        for row_groups in rows_groups {
            row_solution_sets.push(SolutionSet::new(width, row_groups)?);
        }
        for col_groups in cols_groups {
            col_solution_sets.push(SolutionSet::new(height, col_groups)?);
        }
        Ok(Self {
            width,
            height,
            row_solution_sets,
            col_solution_sets,
        })
    }

    pub fn solve(&mut self) -> TrialGrid {
        let mut rows_required = Vec::new();
        for row_solution_set in self.row_solution_sets.iter_mut() {
            row_solution_set.solve();
            rows_required.push(row_solution_set.required());
        };
        let mut cols_required = Vec::new();
        for col_solution_set in self.col_solution_sets.iter_mut() {
            col_solution_set.solve();
            cols_required.push(col_solution_set.required());
        }
        let mut grid = TrialGrid::new(self.width, self.height);
        for (row, row_required) in rows_required.iter().enumerate() {
            for (col, opt_colour) in row_required.iter().enumerate() {
                if let Some(colour) = opt_colour {
                    grid[(row, col)] = TrialColour::Only(*colour);
                }
            }
        }
        for (col, col_required) in cols_required.iter().enumerate() {
            for (row, opt_colour) in col_required.iter().enumerate() {
                if let Some(colour) = opt_colour {
                    grid[(row, col)] = TrialColour::Only(*colour);
                }
            }
        }
        grid
    }
}
