#![allow(unused)]
mod users;

use std::sync::Arc;
use std::sync::RwLock;
use users::Users;
use waseda_moodle::*;

const DEFAULT_USER: &str = "./default-users.json";

#[derive(Clone)]
pub struct AppState {
    users: Arc<RwLock<Users>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(Users::from_file(DEFAULT_USER))),
        }
    }
}

#[tokio::main]
async fn main() -> tide::Result<()> {
    let state = AppState::new();
    let mut app = tide::with_state(state);
    app.at("/login").post(users::login);
    app.at("/register").post(users::register);
    app.listen("127.0.0.1:8080").await?;
    // let session = Session::login("login id", "password").await;
    Ok(())
}
