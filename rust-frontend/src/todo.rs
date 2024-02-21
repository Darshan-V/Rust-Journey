pub struct Todo {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(id: u32, title: &str) -> Todo {
        Todo {
            id,
            title: title.to_string(),
            completed: false,
        }
    }
}

