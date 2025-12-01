use crate::todo::TodoRepository;

pub struct CliCommandDelete {
    id: u32,
}

impl CliCommandDelete {
    pub fn parse(input: &str) -> CliCommandDelete {
        let id = input.trim().parse::<u32>().unwrap_or(0);
        CliCommandDelete { id }
    }

    pub fn handle<R: TodoRepository>(&self, repo: &mut R) {
        let res = repo.delete_todo(self.id);
        match res {
            Ok(_) => {}
            Err(err) => {
                println!("Error deleting todo: {}", err);
            }
            
        }
    }
}
