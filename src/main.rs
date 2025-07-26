mod input;
mod util;

use util::{Difficulty, GuessStatus, HangmanConfig};
use crate::input::Input;

fn main() {
    println!("Welcome to the hangman game! Enter '!' anytime to quit."); 
    println!("Select your difficulty: 
        - (e)asy
        - (m)edium
        - (h)ard 
        - e(x)treme");
    let difficulty = loop {
        let input = Input::new();
        match input.input {
            'e' => break Difficulty::Easy,
            'm' => break Difficulty::Medium,
            'h' => break Difficulty::Hard,
            'x' => break Difficulty::Extreme,
            _ => println!("That is not a valid difficulty. Please try again.")
        }
    };

    let mut hangman = HangmanConfig::new(difficulty);
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
