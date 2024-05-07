// This program will let the user play hangman through the command line. This will show the usage
// of arrays in Rust.

use random_word::Lang;
use rand::Rng;
use core::num;
use std::{io, thread::current};
fn main() 
{
    //Note that the rust compiler infers the type of "word_length"
    let word_length = rand::thread_rng().gen_range(4..=7);
    //Generate random word with length 4..=7
    let word = random_word::gen_len(word_length,Lang::En);
    let word = word.unwrap();

    //Instantiates vector with _ for each char in word
    let mut guessed_letters = vec!['_'; word.len()];
    let mut num_guesses = 6;

    println!("Welcome to Hangman!");

    loop {
        //current_guesses collects the chars from guessed_letters
        let current_guesses = guessed_letters.iter().collect::<String>();
        println!("The Word is: {}", current_guesses);
        println!("You have {} guesses left", num_guesses);

        if current_guesses == word {
            println!("Huray");
            break;
        }
        println!("Enter a letter to guess:");
        //Declare guess variable
        let mut guess = String::new();
        //Takes in actual input for guess var, 
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //Use Pattern matching to return the char if there is one
        //If there is no input 'None' then loop restarted
        let guess: char = match guess.trim().chars().next() {
            Some(cur_guess) if cur_guess.is_ascii_alphabetic() => cur_guess,
            Some(_) => {
                println!("Invalid entry. Enter a letter.");
                continue;
            }
            None => {
                println!("Invalid entry. Enter a letter.");
                continue;
            }
        };

        let mut letter_found = false;
        //If a letter is in word, add to guessed_letters
        for(i,cur_letter) in word.chars().enumerate() {
            if cur_letter == guess {
                guessed_letters[i] = cur_letter;
                letter_found = true;
            }

        }
        if letter_found == false {
            num_guesses -= 1;
            println!("'{}' is not in the word.", guess);
        }

        if num_guesses == 0 {
            println!("Game over. You ran out of guesses. The word was: {}", word);
            break;
        }

    }


}

//Functions use snake_case
fn check_word()
{

}
