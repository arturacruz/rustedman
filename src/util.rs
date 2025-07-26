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
    /// Creates a new game state.
    /// - Target word: picked at random from a premade list.
    /// - Current word: Censored version of the target word.
    /// - Remaining errors: Number ranging from 8 to 2 depending on the difficulty.
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

    /// Handles the guess logic and returns the status of the guess, and if the game has been won
    /// or lost.
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

/// Generates a random word (used by the game config for generating the target word).
fn generate_random_word() -> String {
    let words = [
        "banana", "astronaut", "tiger", "panther"
    ];
    words.choose(&mut rand::rng()).unwrap().to_string()
}
