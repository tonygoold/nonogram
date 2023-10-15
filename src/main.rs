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
    let grid = solver.solve();
    print!("Required solution:\n{}\n", &grid);
}
