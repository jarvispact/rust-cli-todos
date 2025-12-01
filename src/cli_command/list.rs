use crate::todo::{TodoRepository};

pub struct CliCommandList {
    filter_tags: Vec<String>,
}

impl CliCommandList {
    pub fn parse(input: &str) -> CliCommandList {
        let tags = input
            .split_whitespace()
            .filter(|s| s.starts_with("#"))
            .map(|s| s.trim_start_matches('#').to_string())
            .collect::<Vec<String>>();

        CliCommandList { filter_tags: tags }
    }

    pub fn handle<R: TodoRepository>(&self, repo: &mut R) {
        let todos = repo.list_todos(self.filter_tags.clone());

        let todos = match todos {
            Ok(todos) => todos,
            Err(err) => {
                println!("Error retrieving todos: {}", err);
                return;
            }
        };

        if todos.is_empty() {
            println!("No todos found.");
            return;
        }

        for todo in todos {
            let empty = self.filter_tags.is_empty();
            let matches = self.filter_tags.iter().all(|tag| todo.has_tag(tag));
            if empty || matches {
                todo.print();
            }
        }
    }
}
