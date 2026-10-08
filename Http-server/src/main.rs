
use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use std::io::Result;
use std::env;
use std::sync::Mutex;
use dotenvy::from_filename;

use crate::types::user::User;


pub mod controller;
pub mod types;

#[get("/")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Welcome here !")
}

struct AppState {
    pub user_index: Mutex<u64>,
    pub users: Mutex<Vec<User>>
}

#[actix_web::main]
async fn main() -> Result<()> {
    let runtime_env = env::var("RUN_ENV").unwrap_or_else(|_| String::from("local"));
    let file_name = format!(".env.{}", runtime_env);
    from_filename(file_name).ok();

    
    let host = env::var("HOST").unwrap_or_else(|_| String::from("127.0.0.1"));
    let port = env::var("PORT").unwrap_or_else(|_| String::from("3001"));
    let bind_address = format!("{}:{}", host, port);
    println!("running on {runtime_env} enviorenment");

    let app_state = AppState {
        user_index: Mutex::new(1),
        users: Mutex::new(Vec::new())
    };

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(greet)
    })
    .bind(bind_address)?
    .run()
    .await
}