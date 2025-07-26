use std::io::{self, stdin};

pub struct Input {
    pub buffer: char,
    pub size: usize
}

enum InputError {
    IncorrectSize,
    Char
}

impl Input {
    pub fn new() -> Self {
        let mut buffer = String::new();
        loop {
            let result = stdin().read_line(&mut buffer);
            match result {
                Ok(size) => return Self { buffer: String::from(buffer.trim()), size },
                Err(_) => println!("Invalid input. Please try again"),
            };
        } 
    }

    pub fn equals(&self, s: char) -> bool {
        s == self.buffer
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

fn validate_input(buffer: String, size: usize) -> Result<char, InputError> {
   if size != 1 {
       return Err(InputError::IncorrectSize);
   } 
   let char = match buffer.chars().nth(0) {
       Some(char) => char,
       None => return Err(InputError::IncorrectSize), // TODO: Change this error later
   };

   // TODO: Validate if alphabetical and lowercase
   Ok(char)
}
