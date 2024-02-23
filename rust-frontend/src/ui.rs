use crate::file_handling::{read_todos, save_todos};
use crate::todo::Todo;
use std::io::{self, Write};

pub fn print_todos(todos: &Vec<Todo>) {
    //pub -> makes function visible to others
    for todo in todos {
        println!(
            "{} [{}] {}",
            todo.id,
            if todo.completed { "x" } else { " " },
            todo.title
        );
    }
}

pub fn run_todo_app() {
    let mut todos = read_todos();
    let mut next_id: u32 = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;

    loop {
        println!("Todo App");
        println!("1. Add Todo");
        println!("2. List Todos");
        println!("3. Toggle Todo");
        println!("4. Quit");

        print!("Select an option: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<u32>() {
            Ok(1) => {
                print!("Enter todo title: ");
                io::stdout().flush().unwrap();
                let mut title = String::new();
                io::stdin()
                    .read_line(&mut title)
                    .expect("Failed to read line");
                let todo = Todo::new(next_id, title.trim());
                todos.push(todo);
                next_id += 1;
                save_todos(&todos);
            }
            Ok(2) => {
                println!("Todos:");
                print_todos(&todos);
            }
            Ok(3) => {
                print!("Enter todo id to toggle: ");
                io::stdout().flush().unwrap();
                let mut id_input = String::new();
                io::stdin()
                    .read_line(&mut id_input)
                    .expect("Failed to read line");
                if let Ok(id) = id_input.trim().parse::<u32>() {
                    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                        todo.completed = !todo.completed;
                        save_todos(&todos);
                    } else {
                        println!("Todo with id {} not found", id);
                    }
                } else {
                    println!("Invalid id");
                }
            }
            Ok(4) => break,
            _ => println!("Invalid option"),
        }
    }
}
