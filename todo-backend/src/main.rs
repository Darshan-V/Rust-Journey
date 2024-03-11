use actix_web::{web, App, HttpServer, HttpResponse};
use std::sync::{Mutex, Arc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    map: std::collections::HashMap<String, bool>,
}

impl Todo {
    fn new() -> Self {
        Todo {
            map: std::collections::HashMap::new(),
        }
    }

    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("db.json")?;
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}

struct AppState {
    todo: Mutex<Todo>,
}

async fn handle_add(data: web::Data<Arc<AppState>>, item: web::Json<String>) -> HttpResponse {
    let mut todo = data.todo.lock().expect("Failed to acquire lock");
    todo.insert(item.into_inner());
    if let Err(err) = todo.save() {
        return HttpResponse::InternalServerError().body(format!("Error saving todo: {}", err));
    }
    HttpResponse::Ok().body("Todo item added")
}

async fn handle_complete(data: web::Data<Arc<AppState>>, item: web::Json<String>) -> HttpResponse {
    let mut todo = data.todo.lock().expect("Failed to acquire lock");
    if let Some(_) = todo.complete(&item.into_inner()) {
        if let Err(err) = todo.save() {
            return HttpResponse::InternalServerError().body(format!("Error saving todo: {}", err));
        }
        HttpResponse::Ok().body("Todo item completed")
    } else {
        HttpResponse::NotFound().body("Todo item not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize your Todo data structure
    let todo = Todo::new();
    let app_state = Arc::new(AppState {
        todo: Mutex::new(todo),
    });

    // Start the Actix server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&app_state)))
            .route("/add", web::post().to(handle_add))
            .route("/complete", web::post().to(handle_complete))
    })
        .bind("127.0.0.1:8080")?
        .workers(4) // Set the number of worker threads
        .run()
        .await
}
