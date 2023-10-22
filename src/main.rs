use std::fs::File;

use clap::Parser;

use nonograms::blockdisplay::BlockDisplay;
use nonograms::reader::read;

const MAX_ITERS: usize = 1000;

#[derive(Debug, Parser)]
struct Args {
    /// Maximum number of iterations
    #[arg(short, long, default_value_t = MAX_ITERS)]
    count: usize,

    /// The file to parse
    filename: Option<String>,
}

fn main() {
    let args = Args::parse();
    let file = args.filename.map(
        |name| File::open(&name).expect("Unable to read file")
    );
    let mut solver = if let Some(file) = file {
        read(file)
    } else {
        read(std::io::stdin())
    }.expect("Bad input or unsolvable");
    let mut grid = solver.solve_required();
    let mut iterations = 0;
    while iterations < args.count {
        grid = solver.solve_incremental(&grid);
        if grid.solved() {
            break;
        }
        iterations += 1;
    }
    let display = BlockDisplay::new(&grid);
    if grid.solved() {
        print!("After {} iterations, solution:\n{}\n", iterations, &display);
    } else {
        print!("After {} iterations, partial solution:\n{}\n", iterations, &display);
    }
}
