mod cli_command;
mod todo;
mod utils;

use cli_command::{CliCommand, CliCommandSignal};
use todo::{InMemoryTodoStorage, TodoRepository};

fn repl<R: TodoRepository>(todo_storage: &mut R) {
    loop {
        let input: String = utils::get_input();
        let command = CliCommand::parse(&input);
        let signal = command.handle(todo_storage);
        match signal {
            CliCommandSignal::Continue => continue,
            CliCommandSignal::Exit => break,
        }
    }
}

fn main() {
    utils::clear_screen();
    println!("Rust CLI todos v0.1");
    println!("Type 'help' to see available commands.");
    let mut todo_storage = InMemoryTodoStorage::new();
    repl(&mut todo_storage);
}
