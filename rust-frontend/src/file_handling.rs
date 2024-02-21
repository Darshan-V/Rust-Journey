use crate::todo::Todo;
use std::fs;

pub fn read_todos() -> Vec<Todo> {
    match fs::read_to_string("todos.txt") {
        Ok(contents) => contents
            .lines()
            .filter_map(|line| {
                let mut parts = line.split(';');
                let id = parts.next()?.parse::<u32>().ok()?;
                let title = parts.next()?.to_string();
                let completed = parts.next()?.parse::<bool>().ok()?;
                Some(Todo {
                    id,
                    title,
                    completed,
                })
            })
            .collect(),
        Err(_) => Vec::new(),
    }
}

pub fn save_todos(todos: &Vec<Todo>) {
    let content: String = todos
        .iter()
        .map(|todo| format!("{};{};{}", todo.id, todo.title, todo.completed))
        .collect::<Vec<String>>()
        .join("\n");
    fs::write("todos.txt", content).expect("Failed to write todos to file");
}
