use std::iter::Enumerate;

use rand::seq::IndexedRandom;

use crate::input::Input;

#[derive(Debug)]
pub struct HangmanConfig {
    target_word: String,
    pub current_word: String,
    pub remaining_errors: u8,
}

pub enum GuessStatus {
    Win,
    Success,
    Fail,
    Loss
}

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Extreme
}

impl HangmanConfig {
    pub fn new(difficulty: Difficulty) -> Self {
        let errors = match difficulty {
            Difficulty::Easy => 8,
            Difficulty::Medium => 6,
            Difficulty::Hard => 4,
            Difficulty::Extreme => 2,
        };
        let target_word = generate_random_word();
        let current_word = "_".repeat(target_word.len());

        HangmanConfig { 
            target_word, 
            current_word,
            remaining_errors: errors, 
        }
    }

    pub fn handle_guess(&mut self, guess: Input) -> GuessStatus {
        let guess = guess.input;
        let mut correct_guess = false;

        for (i, char) in self.target_word.char_indices() {
            if guess == char {
                self.current_word.replace_range(i..i+1, char.to_string().as_str());
                correct_guess = true;
            }
        }
        if !correct_guess {
            if self.remaining_errors == 0 {
                return GuessStatus::Loss;
            }
            self.remaining_errors -= 1;
            return GuessStatus::Fail;
        }

        if self.current_word == self.target_word {
            return GuessStatus::Win;
        }

        GuessStatus::Success
    }
}

impl Default for HangmanConfig {
    fn default() -> Self {
        HangmanConfig::new(Difficulty::Medium)
    }
}

fn generate_random_word() -> String {
    let words = [
        "banana", "astronaut", "tiger", "panther"
    ];
    words.choose(&mut rand::rng()).unwrap().to_string()
}
