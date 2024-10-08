//This program will let the user play hangman through the command line. This will show the usage
//of vectors, itorators, and pattern matching in Rust.
//Bennett Beltran

//Rand, random_word and standard io crates are used
use rand::Rng;
use random_word::Lang;
use std::io;

fn main() {
    //Note that the rust compiler infers the type of "word_length"
    let word_length = rand::thread_rng().gen_range(4..=7);
    //Generate random word with length 4..=7
    let word = random_word::gen_len(word_length, Lang::En);
    let word: &str = word.unwrap(); //Borrowed variable

    //Instantiates vector with _ for each char in word
    let mut word_progress = vec!['_'; word.len()];
    //Tracks what letters the user has guessed
    let mut guessed_letters = String::new();
    let mut num_guesses = (word_length as f32 * 1.5).ceil() as usize;

    println!("Welcome to Hangman");

    loop {
        //current_guesses collects the chars from word_progress
        let current_guesses = word_progress.iter().collect::<String>();
        println!("The Word is: {}", current_guesses);
        println!("You have {} guesses left", num_guesses);

        if current_guesses == word {
            println!("Congrats, you guessed the word!!");
            break;
        }
        println!("Enter a letter to guess:");
        //Declare guess variable
        let mut guess = String::new();
        //Takes in actual input for guess var,
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //Use Pattern matching to return the char if there is one (and hasn't been guessed before)
        //If there is no input 'None' or invalid input then loop restarted
        let guess: char = match guess.trim().chars().next() {
            Some(cur_guess) if guessed_letters.contains(cur_guess) => {
                println!("You've already guessed that letter. Enter a different letter");
                continue;
            }
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
        //Add the guess to guessed letters String
        guessed_letters.push(guess);

        let mut letter_found = false;
        //If a letter is in word, add to word_progress
        for (i, cur_letter) in word.chars().enumerate() {
            if cur_letter == guess {
                word_progress[i] = cur_letter;
                letter_found = true;
            }
        }
        if letter_found == false {
            num_guesses -= 1;
            println!("'{}' is not in the word.", guess);
        }

        if num_guesses == 0 {
            println!(
                "Game over. \nYou ran out of guesses. The word was: {}", word);
            break;
        }
        println!("Letters you've guessed so far: {}", guessed_letters);
    }
}
