mod input;
mod util;

use util::{HangmanConfig, GuessStatus};
use crate::input::Input;

fn main() {
    println!("Welcome to the hangman game! Enter '!' anytime to quit."); 
    
    let mut hangman = HangmanConfig::default();
    loop {
        println!("{}", hangman.current_word);
        println!("You have {} remaining mistakes.", hangman.remaining_errors);
        println!("Make your guess: ");
        let input = Input::new();
        if input.equals('!') {
            break;
        }

        match hangman.handle_guess(input) {
            GuessStatus::Success => println!("You guessed correctly!"),
            GuessStatus::Fail => println!("Oops, your guess is wrong."),
            GuessStatus::Loss => { 
                println!("You lose!");
                break;
            }
            GuessStatus::Win => {
                println!("You win!");
                break;
            }
        }
        println!();
    }
}
