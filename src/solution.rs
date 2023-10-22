use super::{Colour, Run, SolutionError};
use super::grid::TrialColour;

pub struct Solution {
    groups: Vec<usize>,
    runs: Vec<Run>,
    padding: usize,
}

impl Solution {
    pub fn new(width: usize, groups: &[usize]) -> Result<Self, SolutionError> {
        let min_width = groups.iter().sum::<usize>() + groups.len() - 1;
        let padding = width.checked_sub(min_width)
            .ok_or(SolutionError::NoSolutionExists)?;
        Ok(Solution {
            groups: Vec::from(groups),
            runs: Vec::new(),
            padding,
        })
    }

    pub fn solve_iter(mut self) -> Vec<Solution> {
        if self.groups.is_empty() {
            if self.padding > 0 {
                self.runs.push(Run::white(self.padding));
                self.padding = 0;
            }
            return vec![self];
        }

        let group = self.groups.remove(0);
        let base: usize = if self.runs.is_empty() { 0 } else { 1 };
        (0..=self.padding).map(|padding| {
            let mut runs = self.runs.clone();
            if base + padding > 0 {
                runs.push(Run::white(base + padding));
            }
            runs.push(Run::black(group));
            Solution {
                groups: self.groups.clone(),
                runs,
                padding: self.padding - padding,
            }
        }).collect()
    }

    pub fn is_complete(&self) -> bool {
        self.groups.is_empty() && self.padding == 0
    }

    pub fn iter(&self) -> SolutionIter {
        SolutionIter::new(self)
    }

    pub fn is_compatible(&self, line: &[TrialColour]) -> bool {
        use TrialColour::{Unknown, Any, Maybe, Only};
        return self.iter().zip(line.iter()).all(|(a, b)| {
            match *b {
                Unknown | Any => true,
                Maybe(c) | Only(c) => c == a,
            }
        })
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for run in self.runs.iter() {
            let s = match run.colour {
                Colour::Black => "#".repeat(run.length),
                Colour::White => " ".repeat(run.length)
            };
            f.write_str(&s)?;
        }
        Ok(())
    }
}

pub struct SolutionIter<'a> {
    solution: &'a Solution,
    run_index: usize,
    run_offset: usize,
}

impl<'a> SolutionIter<'a> {
    pub fn new(solution: &'a Solution) -> Self {
        SolutionIter {
            solution,
            run_index: 0,
            run_offset: 0,
        }
    }
}

impl Iterator for SolutionIter<'_> {
    type Item = Colour;

    fn next(&mut self) -> Option<Self::Item> {
        let run = self.solution.runs.get(self.run_index)?;
        self.run_offset += 1;
        if self.run_offset >= run.length {
            self.run_offset = 0;
            self.run_index += 1;
        }
        Some(run.colour)
    }
}
