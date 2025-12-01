use crate::todo::TodoRepository;

pub struct CliCommandCreate {
    content: String,
    filter_tags: Vec<String>,
}

impl CliCommandCreate {
    pub fn parse(input: &str) -> CliCommandCreate {
        let content_end = input.find('#').unwrap_or(input.len());
        let content = input[..content_end].trim();

        let tags = input
            .split_whitespace()
            .filter(|s| s.starts_with("#"))
            .map(|s| s.trim_start_matches('#').to_string())
            .collect::<Vec<String>>();

        CliCommandCreate {
            content: content.to_string(),
            filter_tags: tags,
        }
    }

    pub fn handle<R: TodoRepository>(&self, repo: &mut R) {
        if self.content.is_empty() {
            println!("Cannot create an empty todo.");
            return;
        }

        let res = repo.add_todo(&self.content, self.filter_tags.clone());
        match res {
            Ok(_) => {}
            Err(err) => {
                println!("Error creating todo: {}", err);
            }
            
        }
    }
}
