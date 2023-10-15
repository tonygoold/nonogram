use super::{Colour, Run, SolutionError};

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
            runs.push(Run::white(base + padding));
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

    pub fn flatten(&self) -> Vec<Colour> {
        let mut colours = Vec::new();
        for run in self.runs.iter() {
            for _ in 0..run.length {
                colours.push(run.colour);
            }
        }
        colours
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
