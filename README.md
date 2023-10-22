# About

This is a solver for black and white nonograms. On an M1 Pro MacBook, it can solve an average 20x20 nonogram in around 150 milliseconds, while a tricky 30x30 nonogram takes around 1.5 seconds.

I wrote this for fun, to see if I could write a nonogram solver without using external references. This project is not maintained, although I may continue to optimize it from time to time.

# Usage

You will need to [install cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) to build and run this project.

```
cargo build --release
./target/release/nonograms [<input file>]

# OR

cargo run --release -- [<input file>]

```

If no input file is specified, it will look for a file named `nonogram.txt` in the current directory. You can also specify the maximum by providing a `--count <iterations>` argument. The default limit is 1,000 iterations. The tricky 30x30 nongram mentioned above only needs 35 iterations to solve.

# Input File Format

The input file is a plain text file consisting of two sequences of non-blank lines separated by a blank line.

The first sequence is the list of row constraints from left to right. The second sequence is the list of column constraints from top to bottom. Each line consists of a comma-separated list of numbers with no spaces before/after the comma. A line beginning with a dash is ignored and can be used for comments or to visually separate the input into groups (e.g., dashes between every five lines).

Example:

```
5
1,1,1
5
1
1

5
1,1
3
1,1
3
```

This corresponds to the following nonogram:

```
         1    1
       5 1 3 1 3
      ._ _ _ _ _
    5 |# # # # #
1 1 1 |#   #   #
    5 |# # # # #
    1 |#
    1 |#
```

# Output

The solution is output using `#` to indicate a black cell and `.` to indicate a white cell. For grids larger than 5x5, it will also output row and column separators every five rows/columns. The output for the above nonogram will be:

```
#####
#.#.#
#####
#....
#....
```
