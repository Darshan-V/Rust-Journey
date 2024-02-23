mod file_handling;
mod todo;
mod ui;

use crate::ui::run_todo_app;

fn main() {
    run_todo_app();
}

// init file
// backyard
// ├── Cargo.lock
// ├── Cargo.toml
// └── src
//     ├── garden
//     │   └── vegetables.rs
//     ├── garden.rs
//     └── main.rs
// use crate::garden::vegetables::Asparagus;
// mod -> module
// use -> import module as crate
// pub mod garden;

// fn main() {
//     let plant = Asparagus {};
//     println!("I'm growing {:?}!", plant);
// }
