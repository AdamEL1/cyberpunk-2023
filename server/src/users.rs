use crate::AppState;
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    fs::File,
    hash::{Hash, Hasher},
    io::BufReader,
};
use tide::{prelude::*, Request};

const NB_ATTRIBUTES: usize = 5;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    name: String,
    email: String,
    university: String,
    courses: Vec<String>,
    description: Description,
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: "".into(),
            email: "".into(),
            university: "".into(),
            courses: vec![],
            description: Description::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct Description {
    creativity: isize,
    punctiality: isize,
    responsability: isize,
    organized: isize,
}

impl Description {
    fn new(creativity: isize, punctiality: isize, responsability: isize, organized: isize) -> Self {
        Self {
            creativity,
            punctiality,
            responsability,
            organized,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Course {}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Users(HashMap<UserId, User>);

impl Users {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn from_file(filepath: &str) -> Self {
        let Ok(file) = File::open(filepath) else {
            println!("Error finding {}", filepath);
            return Users::default()
        };
        let users = match serde_json::from_reader(file) {
            Ok(value) => value,
            Err(err) => {
                println!("{}", err);
                Users::default()
            }
        };
        users
    }
    pub fn get(&self, user_id: UserId) -> Option<&User> {
        self.0.get(&user_id)
    }
    pub fn insert(&mut self, user_id: UserId, user: User) {
        self.0.insert(user_id, user);
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct UserInput {
    name: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Eq, PartialEq, Hash)]
pub struct UserId(pub u64);
impl From<UserInput> for UserId {
    fn from(value: UserInput) -> UserId {
        let mut state = DefaultHasher::new();
        value.name.hash(&mut state);
        value.name.hash(&mut state);
        UserId(state.finish())
    }
}

pub async fn register(mut req: Request<AppState>) -> tide::Result {
    let user_input: UserId = req.body_json::<UserInput>().await?.into();
    let read = req.state().users.read().unwrap();
    let user = match read.get(user_input) {
        Some(user) => serde_json::to_string_pretty(user)?,
        None => serde_json::to_string_pretty(&User::default())?,
    };
    Ok(user.into())
}
