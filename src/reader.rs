use std::io::{BufRead, BufReader, Read};

use super::SolutionError;
use super::solver::Solver;

#[derive(Debug)]
pub enum ReaderError {
    IOError(std::io::Error),
    ParseError(std::num::ParseIntError),
    SolutionError(SolutionError),
}

fn read_line(line: &str) -> Result<Vec<usize>, ReaderError> {
    line.split(',').map(|n|
        n.parse::<usize>().map_err(|err| ReaderError::ParseError(err))
    ).collect::<Result<Vec<usize>, _>>()
}

pub fn read<R: Read>(reader: R) -> Result<Solver, ReaderError> {
    let mut rows_groups: Vec<Vec<usize>> = Vec::new();
    let mut cols_groups: Vec<Vec<usize>> = Vec::new();
    let lines = BufReader::new(reader).lines();
    let mut reading_rows = true;
    for line in lines {
        let line = line.map_err(|err| ReaderError::IOError(err))?;
        if line.starts_with('-') {
            continue;
        } else if line.is_empty() {
            reading_rows = false;
            continue;
        }
        let groups = read_line(&line)?;
        if reading_rows {
            rows_groups.push(groups);
        } else {
            cols_groups.push(groups);
        }
    }
    let width = cols_groups.len();
    let height = rows_groups.len();
    Solver::new(width, height, &rows_groups, &cols_groups)
        .map_err(|err| ReaderError::SolutionError(err))
}
