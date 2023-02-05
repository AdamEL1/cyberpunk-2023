#![allow(unused)]
mod algo;
mod courses;
mod interface;
mod prelude;
mod users;
use courses::Courses;
use prelude::*;
use std::sync::Arc;
use std::sync::RwLock;
use tide::http::headers::HeaderValue;
use tide::security::{CorsMiddleware, Origin};
use users::Users;

#[derive(Clone)]
pub struct AppState {
    users: Arc<RwLock<Users>>,
    courses: Arc<RwLock<Courses>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(Users::from_file(DEFAULT_USERS))),
            courses: Arc::new(RwLock::new(Courses::from_file(DEFAULT_COURSES))),
        }
    }
}

#[tokio::main]
async fn main() -> tide::Result<()> {
    let cors = CorsMiddleware::new()
        .allow_methods("GET, POST".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);
    let state = AppState::new();
    let mut app = tide::with_state(state);
    app.with(cors);
    app.at("/users/login").post(users::login);
    app.at("/users/register").post(users::register);
    app.at("/courses/join").post(courses::join);
    app.at("/courses/register").post(courses::register);
    app.listen("127.0.0.1:80").await?;
    Ok(())
}
