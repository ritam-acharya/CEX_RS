use std::sync::Mutex;

use serde::{Deserialize, Serialize};


pub struct User {
    id: u64,
    email: String,
    password: String
}


#[derive(Deserialize, Serialize)]
pub struct SignupInput {
    email: String,
    password: String
}


#[derive(Deserialize, Serialize)]
pub struct SigninInput {
    email: String,
    password: String
}