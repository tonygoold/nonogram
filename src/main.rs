use std::env;
use std::format;
use std::fs::File;

use nonograms::reader::read;

fn main() {
    let filename = env::args()
        .nth(1)
        .unwrap_or("nonogram.txt".to_owned());
    let file = File::open(&filename)
        .expect(format!("Unable to open {}", &filename).as_str());
    let mut solver = read(file)
        .expect("Bad input or unsolvable");
    let mut grid = solver.solve_required();
    print!("Required solution:\n{}\n", &grid);
    let mut solved = false;
    let mut iterations = 0;
    while ! solved && iterations < 10 {
        let next_grid = solver.solve_incremental(&grid);
        solved = next_grid == grid;
        grid = next_grid;
        iterations += 1;
    }
    print!("After {} iterations, solution:\n{}\n", iterations, &grid);
}
