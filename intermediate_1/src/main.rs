// This program will let the user play hangman through the command line. This will show the usage
// of arrays in Rust.

use random_word::Lang;
use std::io;
fn main() 
{
    //let word_lengths = 3..8;
    let word = random_word::gen(Lang::En);
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess);
    println!("Word is: {}",word);
    println!("You guessed: {guess}");
}
