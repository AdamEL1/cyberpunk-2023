#![allow(unused)]
mod users;

use std::sync::Arc;
use std::sync::RwLock;

use tide::prelude::*;
use tide::Request;
use users::User;
use users::Users;

use crate::users::UserId;

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
    // let mut test = Users::default();
    // test.insert(UserId(15029339261059126448), User::default());
    // println!("{}", serde_json::to_string_pretty(&test)?);
    let mut app = tide::with_state(state);
    app.at("/register").post(users::register);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
