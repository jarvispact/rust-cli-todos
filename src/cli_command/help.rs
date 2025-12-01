enum HelpCommand {
    None,
    Valid(String),
    Invalid(String),
}

pub struct CliCommandHelp {
    command: HelpCommand,
}

const VALID_HELP_COMMANDS: [&str; 3] = ["list", "create", "delete"];

impl CliCommandHelp {
    pub fn parse(input: &str) -> CliCommandHelp {
        match input {
            "" => {
                return CliCommandHelp {
                    command: HelpCommand::None,
                };
            }
            cmd if VALID_HELP_COMMANDS.contains(&cmd) => {
                return CliCommandHelp {
                    command: HelpCommand::Valid(cmd.to_string()),
                };
            }
            _ => {
                return CliCommandHelp {
                    command: HelpCommand::Invalid(input.to_string()),
                };
            }
        }
    }
    pub fn handle(&self) {
        match &self.command {
            HelpCommand::None => {
                println!("Available commands:");
                println!("  help [command] - Show help information");
                println!("  list           - List all todos");
                println!("  create         - Create a new todo");
                println!("  exit           - Exit the application");
            }
            HelpCommand::Valid(cmd) => match cmd.as_str() {
                "list" => {
                    println!("list: Lists all todos.");
                }
                "create" => {
                    println!("create: Creates a new todo.");
                }
                "delete" => {
                    println!("delete: Deletes a todo by ID.");
                }
                _ => {}
            },
            HelpCommand::Invalid(cmd) => {
                println!("No help available for command: {}", cmd);
            }
        }
    }
}
