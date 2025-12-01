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
                println!("Rust CLI todos v0.1 - A simple command-line todo manager\n");
                println!("USAGE:");
                println!("  <command> [arguments]\n");
                println!("COMMANDS:");
                println!("  help [command]      Show help information for a specific command");
                println!("  list [#tags]        List all todos, optionally filtered by tags");
                println!("  create [#tags...]   Create a new todo with optional tags");
                println!("  delete              Delete a todo by its ID");
                println!("  exit                Exit the application\n");
                println!("Use 'help <command>' for more information about a specific command.");
            }
            HelpCommand::Valid(cmd) => match cmd.as_str() {
                "list" => {
                    println!("list - List all todos\n");
                    println!("USAGE:");
                    println!("  list [#tag1] [#tag2] ...\n");
                    println!("DESCRIPTION:");
                    println!("  Displays all todos in the repository. You can optionally filter");
                    println!("  the results by specifying one or more tags prefixed with '#'.\n");
                    println!("ARGUMENTS:");
                    println!("  [#tags]  Optional. One or more tags to filter todos by.\n");
                    println!("EXAMPLES:");
                    println!("  list                  List all todos");
                    println!("  list #work            List todos tagged with 'work'");
                    println!("  list #urgent #home    List todos tagged with both 'urgent' and 'home'");
                }
                "create" => {
                    println!("create - Create a new todo\n");
                    println!("USAGE:");
                    println!("  create <description> [#tag1] [#tag2] ...\n");
                    println!("DESCRIPTION:");
                    println!("  Creates a new todo with the specified description. You can");
                    println!("  optionally add tags by prefixing words with '#'.\n");
                    println!("ARGUMENTS:");
                    println!("  <description>  Required. The text description of the todo.");
                    println!("  [#tags]        Optional. One or more tags to associate with the todo.\n");
                    println!("EXAMPLES:");
                    println!("  create Buy groceries");
                    println!("  create Finish report #work #urgent");
                    println!("  create Call mom #personal #family");
                }
                "delete" => {
                    println!("delete - Delete a todo by ID\n");
                    println!("USAGE:");
                    println!("  delete <id>\n");
                    println!("DESCRIPTION:");
                    println!("  Removes a todo from the repository by its unique ID.");
                    println!("  Use the 'list' command to see todo IDs.\n");
                    println!("ARGUMENTS:");
                    println!("  <id>  Required. The numeric ID of the todo to delete.\n");
                    println!("EXAMPLES:");
                    println!("  delete 1    Delete the todo with ID 1");
                    println!("  delete 42   Delete the todo with ID 42");
                }
                _ => {}
            },
            HelpCommand::Invalid(cmd) => {
                println!("No help available for command: '{}'", cmd);
                println!("Use 'help' to see a list of available commands.");
            }
        }
    }
}
