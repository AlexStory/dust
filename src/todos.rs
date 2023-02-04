use serde::{Serialize, Deserialize};

use crate::persist;

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub title: String,
    pub done: bool,
}

impl Todo {
    fn new(title: String) -> Self {
        Self { title, done: false }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todos {
    pub todos: Vec<Todo>,
}


impl Todos {
    pub fn new() -> Result<Self, std::io::Error> {
        let path = persist::get_path();
        persist::read_file(path)
    }

    pub fn add(&mut self, title: String) {
        self.todos.push(Todo::new(title));
    }

    pub fn clear(&mut self) {
        self.todos.clear();
    }

    pub fn update(&mut self, index: usize, title: String) {
        self.todos[index].title = title;
    }

    pub fn complete(&mut self, index: usize) {
        self.todos[index].done = true;
    }

    pub fn uncomplete(&mut self, index: usize) {
        self.todos[index].done = false;
    }

    pub fn delete(&mut self, index: usize) {
        self.todos.remove(index);
    }

    pub fn list(&self) {
        if self.todos.is_empty() {
            println!("No todos");
            return;
        }
        for (i, todo) in self.todos.iter().enumerate() {
            let done_str = if todo.done { "[x]" } else { "[ ]" };
            println!("{}: {} {}", i, done_str, todo.title);
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn::std::error::Error>> {
        let path = persist::get_path();
        persist::write_file(path, self)?;
        Ok(())
    }
}