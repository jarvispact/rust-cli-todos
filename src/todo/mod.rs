mod in_memory_storage;

pub use in_memory_storage::InMemoryTodoStorage;

#[derive(Clone)]
pub struct Todo {
    id: u32,
    content: String,
    tags: Vec<String>,
}

impl Todo {
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(&tag.to_string())
    }
    pub fn print(&self) {
        println!(
            "- ({}) {} [{}]",
            self.id,
            self.content,
            self.tags.join(", ")
        );
    }
}

pub trait TodoRepository {
    fn add_todo(&mut self, content: &str, tags: Vec<String>) -> Todo;
    fn list_todos(&self, filter_tags: Vec<String>) -> Vec<&Todo>;
    fn delete_todo(&mut self, id: u32) -> bool;
}
