use std::io::stdin;

pub struct Input {
    pub buffer: String,
    pub size: usize
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

    pub fn equals(&self, s: &str) -> bool {
        s == self.buffer
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}
