// Rust program that takes in command line arguments and creates a reproducible random maze
// By Nick Kolesar
// Original C++ version written by Mary Elaine Califf and Nick Kolesar
mod disjointset;
mod maze;
use crate::maze::Maze;
use std::env;

/// Main Function:
/// Creates Maze from command line arguments and prints to file.
/// # Command line examples
///
/// ```
/// cargo run 5 5 output.txt 101
/// cargo run 5 5 output.txt 101 1
/// ```
//
// Example above creates the maze:
//   _ _ _ _ _
//    |  _  | |
//  |_    |_| |
//  | |_|  _  |
//  |_    |_  |
//  |_ _|_|_ _
//
pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        panic!(
            "\nUSAGE: {} rows columns outfile randomSeed [1] to stop early, optional\n",
            args[0]
        );
    }

    let mut stop_early = false;

    if args.len() > 5 {
        let stop: u8 = args[5].parse().unwrap();
        if stop == 1 {
            println!("Stopping Early");
            stop_early = true;
        }
    }

    let num_rows: u32 = args[1].parse().unwrap();
    let num_cols: u32 = args[2].parse().unwrap();
    let outfile = args[3].as_str();

    let seed: u64 = args[4].parse().unwrap();

    // small warning to the user about the displaying of the maze after column num 511
    if num_cols > 511 {
        println!("WARNING: Displaying of a maze this size may appear broken if opened in notepad!");
    }

    let mut my_maze = Maze::new(num_rows, num_cols, outfile, seed, stop_early);

    my_maze.generate_maze();

    my_maze.print();
}
