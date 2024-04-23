// Maze class implementation and Direction enum implementation
// By Nick Kolesar

use std::fs::File;
use std::io::prelude::*;
use rand::{rngs::StdRng, RngCore, SeedableRng};
use crate::disjointset;

#[derive(Debug)]
pub enum Direction
{
    North,
    East,
    South,
    West
}


// used for determining the opposite direction
impl Direction
{
    fn value (&self) -> i32
    {
        match *self
        {
            Direction::North => 3,
            Direction::East => 0,
            Direction::South => 1,
            Direction::West => 2
        }
    }
}

// clone attribute used to allow for copies to be made in maze_walls vector
#[derive(Clone, Debug)]
struct CellWalls
{
    east: bool,
    south: bool
}


pub struct Maze 
{
    maze_walls: Vec<CellWalls>,
    num_rows: u32,
    num_columns: u32,
    random_seed: u64,
    stop_early: bool
}


// "Constructor" for Maze class
#[allow(non_snake_case)]
pub fn Maze(rows: u32, columns: u32, seed: u64, stop_early: bool) -> Maze
{
    let num_cells = rows * columns;
    let mut the_maze = Maze{ num_rows: rows, 
                                   num_columns: columns, 
                                   maze_walls: vec![CellWalls{east: true, south: true}; num_cells as usize], 
                                   random_seed: seed, 
                                   stop_early: stop_early};

    the_maze.maze_walls[(num_cells-1) as usize].east = false;
    
    // return the constructed Maze object
    the_maze
}


fn filewrite(buf: &[u8], file: &mut File) 
{ 
    file.write(buf).expect("Write issue detected."); 
}


//#[allow(dead_code)]
impl Maze
{
    /// This function takes in a cell from the maze as well as a direction.
    /// The return value is the adjacent cell in the direction that has been designated.
    pub fn get_adjacent_index(&self, current_cell: u32, direction: &Direction) -> u32
    {
        match direction 
        {
            Direction::North => return (current_cell - self.num_columns) as u32,
            Direction::East => return (current_cell + 1) as u32,
            Direction::South => return (current_cell + self.num_columns) as u32,
            Direction::West => return (current_cell - 1) as u32
        }
    }


    // direction is borrowed here since ownership isn't needed
    /// This function determines if the potential adjacent cell in the maze is actually a valid location
    pub fn is_valid_direction(&self, current_cell: u32, direction: &Direction) -> bool 
    {
        match direction 
        {
            Direction::North => if current_cell / self.num_columns == 0 
                                { 
                                    return false; 
                                } 
                                else { return true; },

            Direction::East =>  if current_cell % self.num_columns == self.num_columns - 1 
                                { 
                                    return false 
                                } 
                                else { return true; },
            
            Direction::South => if current_cell / self.num_columns == self.num_rows - 1 
                                { 
                                    return false; 
                                } 
                                else { return true; },
            
            Direction::West =>  if current_cell % self.num_columns == 0 
                                { 
                                    return false; 
                                } 
                                else { return true; }
        }
    }

    // private helper for getting an enum from an unsigned integer
    fn get_direction(&self, direction_num: u32) -> Direction
    {
        match direction_num
        {
            0 => Direction::East,
            1 => Direction::South,
            2 => Direction::West,
            _ => Direction::North
        }
    }


    #[allow(non_snake_case)]
    pub fn generate_maze(&mut self)
    {
        // initialize variables
        let NUM_DIRECTIONS = 4;
        let num_cells = self.num_rows * self.num_columns;
        let mut my_set = disjointset::DisjointSet(num_cells);
        let mut maze_complete = false;

        let mut rng = StdRng::seed_from_u64(self.random_seed);

        while !maze_complete
        {
            let mut current_cell = rng.next_u32() % num_cells as u32;
            let direction_num = rng.next_u32() % NUM_DIRECTIONS;
            let mut direction = self.get_direction(direction_num);

            // check that direction is valid
            if !self.is_valid_direction(current_cell, &direction)
            {
                let new_direction = (direction.value() + 2) as u32 % NUM_DIRECTIONS;
                
                direction = self.get_direction(new_direction);
            }

            let mut adjacent_cell = self.get_adjacent_index(current_cell, &direction);
            let set_label1 = my_set.find(current_cell as i32);
            let set_label2 = my_set.find(adjacent_cell as i32);

            if set_label1 != set_label2
            {
                // swap current_cell and adjacent cell since we are only changing south and east
                if matches!(direction, Direction::North) || matches!(direction, Direction::West)
                {
                    let temp = current_cell;
                    current_cell = adjacent_cell;
                    adjacent_cell = temp;
                }

                // break walls
                if matches!(direction, Direction::North) || matches!(direction, Direction::South)
                {
                    self.maze_walls[current_cell as usize].south = false;
                }
                else {
                    self.maze_walls[current_cell as usize].east = false;
                }

                if my_set.union(current_cell as i32, adjacent_cell as i32) || self.stop_early && my_set.find(0) == my_set.find((num_cells-1) as i32)
                {
                    maze_complete = true;
                }
            }

        }
    }


    /// This function generates the output of the maze and writes it to the user defined file
    pub fn print(&self, filename: &str)
    {        
        // variable to perform file io
        let mut file = File::create(filename).expect("Could not open file.");
        
        // top border of maze
        for _ in 0..self.num_columns
        {
            filewrite(b" _", &mut file);
        }

        filewrite(b"\n", &mut file);

        for i in 0..self.num_rows
        {
            let cellbase = i * self.num_columns;

            if i == 0 { 
                filewrite(b" ", &mut file); 
            }
            else {
                filewrite(b"|", &mut file);
            }
            // fill in maze row by row
            for j in 0..self.num_columns
            {
                if self.maze_walls[(cellbase + j) as usize].south {
                    filewrite(b"_", &mut file);
                }
                else {
                    filewrite(b" ", &mut file);
                }
                if self.maze_walls[(cellbase + j) as usize].east {
                    filewrite(b"|", &mut file);
                }
                else {
                    filewrite(b" ", &mut file);
                }
            }
            filewrite(b"\n", &mut file);
        }
    }
}
