
use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use std::io::Result;

#[get("/")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Welcome here !")
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet)
    })
    .bind("127.0.0.1:3001")?
    .run()
    .await
}