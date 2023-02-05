use crate::{
    algo::find_matches,
    interface::{CourseRegister, JoinCourseInput, StateResult, UserInput, UserRegister},
    prelude::*,
    AppState,
};
use std::{
    collections::{
        hash_map::{DefaultHasher, Iter},
        HashMap,
    },
    fs::{self, File},
    hash::{Hash, Hasher},
};
use tide::{prelude::*, Request};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    name: String,
    email: String,
    school: String,
    pub courses: Vec<String>,
    pub description: Description,
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: "".into(),
            email: "".into(),
            school: "".into(),
            courses: vec![],
            description: Description::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Description {
    creativity: isize,
    punctuality: isize,
    responsability: isize,
    organized: isize,
}

impl Description {
    fn new(creativity: isize, punctuality: isize, responsability: isize, organized: isize) -> Self {
        Self {
            creativity,
            punctuality,
            responsability,
            organized,
        }
    }
    pub fn to_array(&self) -> [isize; 4] {
        [
            self.creativity,
            self.punctuality,
            self.responsability,
            self.organized,
        ]
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Users(HashMap<UserId, User>);

impl Users {
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
    pub fn to_file(&self, filepath: &str) {
        let data = serde_json::to_string_pretty(self).expect("Serialization failed.");
        fs::write(filepath, data).expect("Can't save data.");
    }
    pub fn get(&self, user_id: UserId) -> Option<&User> {
        self.0.get(&user_id)
    }
    pub fn insert(&mut self, user_id: UserId, user: User) {
        self.0.insert(user_id, user);
        self.to_file(DEFAULT_USERS);
    }
    pub fn iter(&self) -> Iter<UserId, User> {
        self.0.iter()
    }
}

impl From<UserRegister> for User {
    fn from(user: UserRegister) -> Self {
        Self {
            name: user.name,
            email: user.email,
            school: user.school,
            courses: user.courses.iter().map(|x| x.title.clone()).collect(),
            description: Description::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Eq, PartialEq, Hash)]
pub struct UserId(pub u64);
impl From<UserInput> for UserId {
    fn from(value: UserInput) -> UserId {
        let mut state = DefaultHasher::new();
        value.name.hash(&mut state);
        value.password.hash(&mut state);
        UserId(state.finish())
    }
}

impl From<UserRegister> for UserId {
    fn from(value: UserRegister) -> UserId {
        let mut state = DefaultHasher::new();
        value.name.hash(&mut state);
        value.password.hash(&mut state);
        UserId(state.finish())
    }
}

impl From<&JoinCourseInput> for UserId {
    fn from(value: &JoinCourseInput) -> UserId {
        let mut state = DefaultHasher::new();
        value.name.hash(&mut state);
        value.password.hash(&mut state);
        UserId(state.finish())
    }
}

impl StateResult {
    pub fn new(state: bool) -> String {
        serde_json::to_string(&Self { state }).unwrap()
    }
}

pub async fn register(mut req: Request<AppState>) -> tide::Result {
    println!("Received POST at {}", req.url());
    let user = match req.body_json::<UserRegister>().await {
        Ok(value) => value,
        Err(err) => {
            println!("An error occured: {}", err);
            return Ok(StateResult::new(false).into());
        }
    };
    let user_id: UserId = user.clone().into();
    {
        let mut write = req.state().courses.write().unwrap();
        write.new_user(&user, user_id);
    }
    {
        let mut write = req.state().users.write().unwrap();
        write.insert(user_id, user.into());
    }
    Ok(StateResult::new(true).into())
}

pub async fn login(mut req: Request<AppState>) -> tide::Result {
    println!("Received POST at {}", req.url());
    let user_id: UserId = req.body_json::<UserInput>().await?.into();
    let read = req.state().users.read().unwrap();
    let user = match read.get(user_id) {
        Some(user) => serde_json::to_string_pretty(user)?,
        None => serde_json::to_string_pretty(&User::default())?,
    };
    Ok(user.into())
}

pub async fn select(mut req: Request<AppState>) -> tide::Result {
    println!("Received POST at {}", req.url());
    let input = match req.body_json::<JoinCourseInput>().await {
        Ok(value) => value,
        Err(err) => {
            println!("{}", err);
            return Ok("".into());
        }
    };
    let user_id: UserId = (&input).into();
    let read_user = req.state().users.read().unwrap();
    let user = read_user.get(user_id).unwrap();
    let read_courses = req.state().courses.read().unwrap();
    let course = read_courses.get(input.course.title).unwrap();
    let other_users = course
        .users
        .iter()
        .filter(|x| **x != user_id)
        .map(|user| read_user.get(*user).unwrap().clone())
        .map(|mut user| {
            user.email = format!("mailto:{}?Subject=Partenaire%20de%20laboratoire%20pour%20le%20cours%20{}&Body=Je%20suis%20intéressé%20à%20être%20votre%20partenaire%20de%20laboratoire%20pour%20le%20cours%20.%0A%0A{}", user.email, course.name, user.name);
            user
        })
        .collect();
    let result = find_matches(user.clone(), other_users, course.clone());
    Ok(serde_json::to_string_pretty(&result)?.into())
}
