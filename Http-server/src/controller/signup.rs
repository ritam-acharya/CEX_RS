use actix_web::{Responder, post};


#[post("/signup")]
async fn signup() -> impl Responder {
    "signup"
}