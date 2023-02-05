use crate::{
    interface::{CourseRegister, JoinCourseInput, StateResult, UserInput, UserRegister},
    prelude::{DEFAULT_COURSES, DEFAULT_USERS},
    users::{User, UserId, Users},
    AppState,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    fs::{self, File},
    hash::{Hash, Hasher},
};
use tide::{prelude::*, Request};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Course {
    pub name: String,
    pub users: Vec<UserId>,
}

impl Course {
    fn with_user(name: String, user_id: UserId) -> Self {
        Self {
            name,
            users: vec![user_id],
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Courses(HashMap<String, Course>);

impl Courses {
    pub fn from_file(filepath: &str) -> Self {
        let Ok(file) = File::open(filepath) else {
            println!("Error finding {}", filepath);
            return Courses::default()
        };
        let users = match serde_json::from_reader(file) {
            Ok(value) => value,
            Err(err) => {
                println!("{}", err);
                Courses::default()
            }
        };
        users
    }
    pub fn to_file(&self, filepath: &str) {
        let data = serde_json::to_string_pretty(self).expect("Serialization failed.");
        fs::write(filepath, data).expect("Can't save data.");
    }
    pub fn get(&self, course_id: String) -> Option<&Course> {
        self.0.get(&course_id)
    }
    pub fn insert(&mut self, course_id: String, course: Course) {
        self.0.insert(course_id, course);
        self.to_file(DEFAULT_COURSES);
    }
    pub fn new_user(&mut self, user: &UserRegister, user_id: UserId) {
        for course in user.courses.iter() {
            self.add_user(user_id, &course.title);
        }
        self.to_file(DEFAULT_COURSES);
    }
    pub fn add_user(&mut self, user_id: UserId, course: &String) {
        match self.0.get_mut(course) {
            Some(value) => {
                if !value.users.contains(&user_id) {
                    value.users.push(user_id)
                }
            }
            None => self.insert(course.clone(), Course::with_user(course.clone(), user_id)),
        };
        self.to_file(DEFAULT_COURSES);
    }
}

impl From<CourseRegister> for Course {
    fn from(value: CourseRegister) -> Self {
        Self {
            name: value.title,
            users: vec![],
        }
    }
}

pub async fn register(mut req: Request<AppState>) -> tide::Result {
    println!("Received POST for {}", req.url());
    let course = match req.body_json::<CourseRegister>().await {
        Ok(value) => value,
        Err(err) => return Ok(format!("{}\n", err).into()),
    };
    let mut write = req.state().courses.write().unwrap();
    write.insert(course.title.clone(), course.into());
    Ok("Ok\n".into())
}

pub async fn join(mut req: Request<AppState>) -> tide::Result {
    println!("Received POST at {}", req.url());
    let input: JoinCourseInput = req.body_json().await?;
    let user_register = UserInput {
        name: input.name,
        password: input.password,
    };
    let user_id: UserId = user_register.into();
    let mut write = req.state().courses.write().unwrap();
    write.add_user(user_id, &input.course.title);
    let read = req.state().users.read().unwrap();
    read.to_file(DEFAULT_USERS);
    Ok(StateResult::new(true).into())
}
