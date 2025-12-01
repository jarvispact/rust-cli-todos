use super::{Todo, TodoRepository};

pub struct InMemoryTodoStorage {
    todos: Vec<Todo>,
    next_id: u32,
}

impl InMemoryTodoStorage {
    pub fn new() -> Self {
        InMemoryTodoStorage {
            todos: Vec::new(),
            next_id: 1,
        }
    }
}

impl TodoRepository for InMemoryTodoStorage {
    fn add_todo(&mut self, content: &str, tags: Vec<String>) -> Result<Todo, String> {
        let todo = Todo {
            id: self.next_id,
            content: content.to_string(),
            tags,
        };
        self.todos.push(todo);
        self.next_id += 1;
        Ok(self.todos.last().unwrap().clone())
    }

    fn list_todos(&self, filter_tags: Vec<String>) -> Result<Vec<&Todo>, String> {
        Ok(self
            .todos
            .iter()
            .filter(|todo| {
                filter_tags.is_empty() || filter_tags.iter().all(|tag| todo.tags.contains(tag))
            })
            .collect())
    }

    fn delete_todo(&mut self, id: u32) -> Result<(), String> {
        if let Some(pos) = self.todos.iter().position(|todo| todo.id == id) {
            self.todos.remove(pos);
            Ok(())
        } else {
            Err("Todo not found".to_string())
        }
    }
}
