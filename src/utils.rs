use std::io::{self, Write};

pub fn clear_screen() {
    // Clear the terminal screen
    print!("\x1B[2J\x1B[1;1H");
}

pub fn get_input() -> String {
    print!("> ");
    let flush_result = io::stdout().flush();
    if flush_result.is_err() {
        return String::new();
    }

    let mut input: String = String::new();
    let rl_result = io::stdin().read_line(&mut input);
    if rl_result.is_err() {
        return String::new();
    }

    input.trim().to_string()
}