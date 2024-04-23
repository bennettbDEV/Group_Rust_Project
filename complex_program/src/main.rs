// Rust program that takes in command line arguments and creates a reproducible random maze
// By Nick Kolesar 

pub mod disjointset;
pub mod maze;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5
    {
        panic!("\nUSAGE: {} rows columns outfile randomSeed [1] to stop early, optional\n", args[0]);
    }

    let mut stop_early = false;

    if args.len() > 5
    {
        let stop: u8 = args[5].parse().unwrap();
        if stop == 1
        {
            println!("Stopping Early");
            stop_early = true;
        }
    }

    let num_rows: u32 = args[1].parse().unwrap();
    let num_cols: u32 = args[2].parse().unwrap();
    let outfile = args[3].as_str();

    let seed: u64 = args[4].parse().unwrap();

    let mut my_maze = maze::Maze(num_rows, num_cols, seed, stop_early);

    my_maze.generate_maze();
    
    my_maze.print(outfile);
    
}
