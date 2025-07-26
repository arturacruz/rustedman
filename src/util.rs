use rand::seq::IndexedRandom;

#[derive(Debug)]
pub struct HangmanConfig {
    target_word: String,
    current_word: String,
    remaining_errors: u8,
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

    pub fn handle_guess(&self, guess: char) {
        // for char in 
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
