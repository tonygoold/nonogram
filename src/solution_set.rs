use super::{Colour, SolutionError};
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
        while ! self.partial.is_empty() {
            let solution = self.partial.remove(0);
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

    pub fn required(&mut self) -> Vec<Option<Colour>> {
        let mut solved = self.solved.iter();
        let initial = solved.next().expect("Empty solution set");
        let mut reqs: Vec<Option<Colour>> = initial.flatten().into_iter()
            .map(Option::Some).collect();
        for solution in self.solved.iter().skip(1) {
            for (i, colour) in solution.flatten().into_iter().enumerate() {
                if let Some(c) = reqs[i] {
                    if c != colour {
                        reqs[i] = None;
                    }
                }
            }
        }
        reqs
    }
}
