use std::io::{self, Write};

pub fn prompt_user(message: &str) -> String {
    print!("{} ", message);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
