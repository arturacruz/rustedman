mod input;
mod util;

use util::HangmanConfig;
use crate::input::Input;

fn main() {
    println!("Welcome to the hangman game! Enter '!' anytime to quit."); 
    
    let hangman = HangmanConfig::default();
    println!("{hangman:?}");

    loop {
        let input = Input::new();
        if input.equals('!') {
            break;
        }
    }
}
