use super::{Colour, SolutionError};
use super::grid::TrialColour;
use super::solution::Solution;

pub struct SolutionSet {
    partial: Vec<Solution>,
    solved: Vec<Solution>,
}

impl SolutionSet {
    pub fn new(width: usize, groups: &[usize]) -> Result<Self, SolutionError> {
        Ok(SolutionSet {
            partial: vec![Solution::new(width, groups)?],
            solved: Vec::new(),
        })
    }

    pub fn solve(&mut self) {
        while let Some(solution) = self.partial.pop() {
            let solutions = solution.solve_iter();
            for solution in solutions {
                if solution.is_complete() {
                    self.solved.push(solution);
                } else {
                    self.partial.push(solution);
                }
            }
        }
    }

    pub fn remove_incompatible(&mut self, line: &[TrialColour]) {
        let mut removed: Vec<usize> = Vec::new();
        for (i, solution) in self.solved.iter().enumerate() {
            if ! solution.is_compatible(line) {
                removed.push(i);
            }
        }
        while let Some(i) = removed.pop() {
            self.solved.swap_remove(i);
        }
    }

    pub fn solved(&self) -> &[Solution] {
        &self.solved
    }

    pub fn required(&mut self) -> Vec<Option<Colour>> {
        let mut solved = self.solved.iter();
        let initial = solved.next().expect("Empty solution set");
        let mut reqs: Vec<Option<Colour>> = initial.iter()
            .map(Option::Some).collect();
        for solution in self.solved.iter().skip(1) {
            for (i, colour) in solution.iter().enumerate() {
                if let Some(c) = reqs[i] {
                    if c != colour {
                        reqs[i] = None;
                    }
                }
            }
        }
        reqs
    }

    pub fn remove(&mut self, index: usize) {
        self.solved.remove(index);
    }
}
