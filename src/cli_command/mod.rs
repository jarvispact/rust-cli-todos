mod create;
mod delete;
mod help;
mod list;

use crate::todo::TodoRepository;
use create::CliCommandCreate;
use delete::CliCommandDelete;
use help::CliCommandHelp;
use list::CliCommandList;

pub enum CliCommand {
    Empty,
    Help(CliCommandHelp),
    Exit,
    List(CliCommandList),
    Create(CliCommandCreate),
    Delete(CliCommandDelete),
}

pub enum CliCommandSignal {
    Continue,
    Exit,
}

impl CliCommand {
    pub fn parse(input: &str) -> CliCommand {
        let input = input.trim();

        if input.starts_with("help") {
            return CliCommand::Help(CliCommandHelp::parse(input[4..].trim()));
        }

        if input.starts_with("exit") {
            return CliCommand::Exit;
        }

        if input.starts_with("list") {
            return CliCommand::List(CliCommandList::parse(input[4..].trim()));
        }

        if input.starts_with("create") {
            return CliCommand::Create(CliCommandCreate::parse(input[6..].trim()));
        }

        if input.starts_with("delete") {
            return CliCommand::Delete(CliCommandDelete::parse(input[6..].trim()));
        }

        CliCommand::Empty
    }

    pub fn handle<R: TodoRepository>(&self, repo: &mut R) -> CliCommandSignal {
        match self {
            CliCommand::Help(cmd) => {
                cmd.handle();
                return CliCommandSignal::Continue;
            }
            CliCommand::Exit => {
                println!("Bye!");
                return CliCommandSignal::Exit;
            }
            CliCommand::List(cmd) => {
                cmd.handle(repo);
                return CliCommandSignal::Continue;
            }
            CliCommand::Create(cmd) => {
                cmd.handle(repo);
                return CliCommandSignal::Continue;
            }
            CliCommand::Delete(cmd) => {
                cmd.handle(repo);
                return CliCommandSignal::Continue;
            }
            CliCommand::Empty => {
                return CliCommandSignal::Continue;
            }
        }
    }
}
