#![warn(clippy::all, clippy::pedantic, clippy::perf)]
use clap::{Parser, Subcommand};
use todos::Todos;

mod persist;
mod todos;

#[derive(Parser)]
#[command(author, about, version, arg_required_else_help = true, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all todos
    List,
    /// Add a new todo
    Add {
        /// name of the todo
        title: String,
    },
    /// Complete a todo
    Complete {
        /// index of the todo
        index: usize,
    },
    /// Uncomplete a todo
    Uncomplete {
        /// index of the todo
        index: usize,
    },
    /// Delete a todo
    Delete {
        /// index of the todo
        index: usize,
    },
    /// Clear all todos
    Clear,
    /// Update a todo
    Update {
        /// index of the todo
        index: usize,
        /// name of the todo
        title: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let todos;

    match Todos::new() {
        Ok(t) => todos = t,
        Err(e) => {
            println!("{e}");
           return
        }
    }

    match &cli.command {
        Some(Commands::List) =>  list(&todos),
        Some(Commands::Clear) => clear(todos),
        Some(Commands::Add { title }) => add(todos, title),
        Some(Commands::Complete { index }) => complete(todos, *index),
        Some(Commands::Uncomplete { index }) => uncomplete(todos, *index),
        Some(Commands::Delete { index }) => delete(todos, *index),
        Some(Commands::Update { index, title }) => update(todos, *index, title),
        None => {}
    }
}

fn list(todos: &Todos) {
    todos.list();
}

fn add(mut todos: Todos, title: &str) {
    todos.add(title.to_string());
    match todos.save() {
        Ok(_) => {}
        Err(e) => println!("{e}"),
    }
}

fn clear(mut todos: Todos) {
    todos.clear();
    match todos.save() {
        Ok(_) => println!("Todos cleared"),
        Err(e) => println!("{e}"),
    }
}

fn complete(mut todos: Todos, index: usize) {
    if index >= todos.todos.len() {
        println!("Index out of bounds");
        return;
    }
    todos.complete(index);
    match todos.save() {
        Ok(_) => println!("Item completed"),
        Err(e) => println!("{e}"),
    }
}

fn uncomplete(mut todos: Todos, index: usize) {
    if index >= todos.todos.len() {
        println!("Index out of bounds");
        return;
    }
    todos.uncomplete(index);
    match todos.save() {
        Ok(_) => println!("Item uncompleted"),
        Err(e) => println!("{e}"),
    }
}

fn update(mut todos: Todos, index: usize, title: &str) {
    if index >= todos.todos.len() {
        println!("Index out of bounds");
        return;
    }
    todos.update(index, title.to_string());
    match todos.save() {
        Ok(_) => println!("Item updated"),
        Err(e) => println!("{e}"),
    }
}

fn delete(mut todos: Todos, index: usize) {
    if index >= todos.todos.len() {
        println!("Index out of bounds");
        return;
    }
    todos.delete(index);
    match todos.save() {
        Ok(_) => println!("Item deleted"),
        Err(e) => println!("{e}"),
    }
}