// A program that converts binary numbers to base 10 numbers. This will demonstrate loop usage in Rust as well as matching. This will be done through command line arguments.
// fix panic if there are no command line arguments and add some comments

use std::env;

fn main()
{
    let args: Vec<String> = env::args().collect();
    let val_to_convert = &args[1];
    print!("Original binary number: {val_to_convert}\n");
    let binary_number = find_binary(val_to_convert);
    print!("Base 10 number: {binary_number}\n");


}

fn find_binary(val_to_convert: &str) -> i32 {
    let binary_vector: Vec<char> = val_to_convert.chars().collect();

    let mut current_val = 0;
    let mut current_index = 0;

    for num in binary_vector.iter().rev() {
        match num {
            '0' => {},
            _ => current_val = current_val + 2_i32.pow(current_index),
        }
        current_index += 1;
    }

    return current_val;
}

