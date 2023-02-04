use std::{ env, fs::OpenOptions};

use crate::todos::Todos;

pub fn get_path() -> String {
    let path = env::current_exe().unwrap();
    path.parent().unwrap().join("todos.json").to_str().unwrap().to_string()
}

pub fn read_file(path: String) -> Result<Todos, std::io::Error> {
    let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        match serde_json::from_reader(f) {
            Ok(todos) => Ok(todos),
            Err(e) if e.is_eof() => Ok(Todos { todos: vec![] }),
            Err(e) => panic!("{e}"),
        }
}

pub fn write_file(path: String, todos: &Todos) -> Result<(), Box<dyn::std::error::Error>> {
    let f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

    serde_json::to_writer_pretty(f, todos)?;
    Ok(())
}