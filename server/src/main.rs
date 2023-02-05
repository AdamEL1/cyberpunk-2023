#![allow(unused)]
mod courses;
mod interface;
mod users;
use courses::Courses;
use std::sync::Arc;
use std::sync::RwLock;
use users::Users;

const DEFAULT_USER: &str = "./default-users.json";
const DEFAULT_COURSES: &str = "./default-courses.json";

#[derive(Clone)]
pub struct AppState {
    users: Arc<RwLock<Users>>,
    courses: Arc<RwLock<Courses>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(Users::from_file(DEFAULT_USER))),
            courses: Arc::new(RwLock::new(Courses::from_file(DEFAULT_COURSES))),
        }
    }
}

#[tokio::main]
async fn main() -> tide::Result<()> {
    let state = AppState::new();
    // for (user_id, _) in state.users.read().unwrap().iter() {
    //     state.courses.
    // }
    let mut app = tide::with_state(state);
    app.at("/users/login").post(users::login);
    app.at("/users/register").post(users::register);
    // app.at("/users/join_course").post(users::register);
    app.at("/courses/register").post(courses::register);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
