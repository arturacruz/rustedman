use std::io::stdin;

pub struct Input {
    pub input: char,
}

enum InputError {
    IncorrectSize,
    InvalidType
}

impl Input {
    /// The function only returns a valid Input, but loops and repeats the validation if it is not
    /// yet valid.
    pub fn new() -> Self {
        let mut buffer = String::new();
        loop {
            let result = stdin().read_line(&mut buffer);
            match result {
                Ok(size) => {
                    // Removing new line character
                    buffer = buffer.trim().to_string();
                    match validate_input(&mut buffer, size) {
                        Ok(char) => return Self { input: char },
                        Err(err) => print_input_error(err),
                    } 
                },
                Err(_) => println!("Invalid input. Please try again"),
            };
        } 
    }

    /// Checks if the chars are equal.
    pub fn equals(&self, s: char) -> bool {
        s == self.input
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

/// Private function used to validate the user input, garanteeing it obeys the following rules:
/// - Is only one char.
/// - Is alphabetical or exclamation mark
/// 
/// It also makes the character lowercase, if not the case already.
fn validate_input(buffer: &mut str, size: usize) -> Result<char, InputError> {
    // Taking account for the new line character.
    if size != 2 {
        return Err(InputError::IncorrectSize);
    } 
    let mut char = match buffer.chars().next() {
        Some(char) => char,
        None => return Err(InputError::IncorrectSize), // TODO: Change this error later
    };
 
    if !char.is_alphabetic() && char != '!' {
        return Err(InputError::InvalidType);
    }
    if char.is_alphabetic() {
        char::make_ascii_lowercase(&mut char);
    }
    Ok(char)
}

/// Prints the correct message to the user based on their input mistake.
fn print_input_error(err: InputError) {
    match err {
        InputError::IncorrectSize => println!("Invalid input. Make sure it is a single character."),
        InputError::InvalidType => println!("Invalid input. Make sure it is alphabetical (or '!' to quit)"),
    };
}
