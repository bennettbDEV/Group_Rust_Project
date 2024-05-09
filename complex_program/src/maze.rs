// struct and implementation of the Maze Object
// By Nick Kolesar
// Original C++ version written by Mary Elaine Califf and Nick Kolesar
use crate::disjointset::DisjointSet;
use rand::{rngs::StdRng, RngCore, SeedableRng};
use std::fs::File;
use std::io::prelude::*;

/// Enumeration of the four Cardinal directions
#[derive(Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

/// implementation used for determining the opposite direction of the current Direction
impl Direction {
    /// value is used to allow an enum to be substituted for an i32
    fn value(&self) -> i32 {
        match *self {
            Direction::North => 3,
            Direction::East => 0,
            Direction::South => 1,
            Direction::West => 2,
        }
    }
}

// clone attribute used to allow for copies to be made in maze_walls vector
#[derive(Clone, Debug)]
/// Custom type for keeping track of south and east walls of a cell
struct CellWalls {
    east: bool,
    south: bool,
}

/// struct holding information pertaining to the Maze Object
pub struct Maze {
    maze_walls: Vec<CellWalls>,
    num_rows: u32,
    num_columns: u32,
    random_seed: u64,
    stop_early: bool,
    file: File,
}

//#[allow(dead_code)]
impl Maze {
    /// "Constructor" for Maze class
    /// This function builds a Maze instance with the values aquired through the command line
    #[allow(non_snake_case)]
    pub fn new(rows: u32, columns: u32, outfile: &str, seed: u64, stop_early: bool) -> Self {
        let num_cells = rows as usize * columns as usize;
        // mut here makes all fields mutable
        let mut the_maze = Maze {
            num_rows: rows,
            num_columns: columns,
            maze_walls: vec![
                CellWalls {
                    east: true,
                    south: true
                };
                num_cells
            ],
            random_seed: seed,
            stop_early: stop_early,
            file: File::create(outfile).expect("Could not open file."),
        };

        the_maze.maze_walls[num_cells - 1].east = false;

        // return the constructed Maze object
        the_maze
    }

    /// This function takes in a cell from the maze as well as a direction.
    /// The return value is the adjacent cell in the direction that has been designated.
    pub fn get_adjacent_index(&self, current_cell: u32, direction: &Direction) -> u32 {
        match direction {
            Direction::North => return (current_cell - self.num_columns) as u32,
            Direction::East => return (current_cell + 1) as u32,
            Direction::South => return (current_cell + self.num_columns) as u32,
            Direction::West => return (current_cell - 1) as u32,
        }
    }

    // direction is borrowed here since ownership isn't needed
    /// This function determines if the potential adjacent cell in the maze is actually a valid location
    pub fn is_valid_direction(&self, current_cell: u32, direction: &Direction) -> bool {
        match direction {
            Direction::North => {
                if current_cell / self.num_columns == 0 {
                    false
                } else {
                    true
                }
            }
            Direction::West => {
                if current_cell % self.num_columns == 0 {
                    false
                } else {
                    true
                }
            }
            Direction::East => {
                if current_cell % self.num_columns == self.num_columns - 1 {
                    false
                } else {
                    true
                }
            }
            Direction::South => {
                if current_cell / self.num_columns == self.num_rows - 1 {
                    false
                } else {
                    true
                }
            }
        }
    }

    // private helper for getting an enum from an unsigned integer
    fn get_direction(&self, direction_num: u8) -> Direction {
        match direction_num {
            0 => Direction::East,
            1 => Direction::South,
            2 => Direction::West,
            _ => Direction::North,
        }
    }

    #[allow(non_snake_case)]
    /// This function creates the maze itself by randomly selecting a location and an adjacent cell
    /// If the cell and it's neighbor are already in the same set then nothing is done
    /// If the cells are not in the same set then they are unionized and the maze is checked for completion
    pub fn generate_maze(&mut self) {
        // initialize variables
        let NUM_DIRECTIONS = 4;
        let num_cells = self.num_rows * self.num_columns;
        let mut my_set = DisjointSet::new(num_cells);
        let mut maze_complete = false;
        let mut rng = StdRng::seed_from_u64(self.random_seed);

        while !maze_complete {
            let mut current_cell = rng.next_u32() % num_cells as u32;
            let direction_num = rng.next_u32() % NUM_DIRECTIONS;
            let mut direction = self.get_direction(direction_num as u8);

            // check that direction is valid
            if !self.is_valid_direction(current_cell, &direction) {
                let new_direction = (direction.value() + 2) as u32 % NUM_DIRECTIONS;

                direction = self.get_direction(new_direction as u8);
            }

            let mut adjacent_cell = self.get_adjacent_index(current_cell, &direction);
            let set_label1 = my_set.find(current_cell as i32);
            let set_label2 = my_set.find(adjacent_cell as i32);

            if set_label1 != set_label2 {
                // swap current_cell and adjacent cell since we are only changing south and east
                if matches!(direction, Direction::North) || matches!(direction, Direction::West) {
                    let temp = current_cell;
                    current_cell = adjacent_cell;
                    adjacent_cell = temp;
                }

                // break walls
                if matches!(direction, Direction::North) || matches!(direction, Direction::South) {
                    self.maze_walls[current_cell as usize].south = false;
                } else {
                    self.maze_walls[current_cell as usize].east = false;
                }

                if my_set.union(current_cell as i32, adjacent_cell as i32)
                    || self.stop_early && my_set.find(0) == my_set.find((num_cells - 1) as i32)
                {
                    maze_complete = true;
                }
            }
        }
    }

    /// This is a private helper function used to make writing to file cleaner
    fn filewrite(&mut self, buf: &[u8]) {
        self.file.write(buf).expect("Write issue detected.");
    }

    /// This function generates the output of the maze and writes it to the user defined file
    pub fn print(&mut self) {
        // top border of maze
        for _ in 0..self.num_columns {
            self.filewrite(b" _");
        }

        // start writing maze on new line
        self.filewrite(b"\n");

        // draw maze
        for i in 0..self.num_rows {
            let cellbase = i * self.num_columns;

            // leftmost wall
            if i == 0 {
                self.filewrite(b" ");
            } else {
                self.filewrite(b"|");
            }

            // fill in rest of maze row
            for j in 0..self.num_columns {
                if self.maze_walls[(cellbase + j) as usize].south {
                    self.filewrite(b"_");
                } else {
                    self.filewrite(b" ");
                }
                if self.maze_walls[(cellbase + j) as usize].east {
                    self.filewrite(b"|");
                } else {
                    self.filewrite(b" ");
                }
            }
            // set write pointer to next line
            self.filewrite(b"\n");
        }
    }
}
