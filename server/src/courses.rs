use crate::{
    interface::{CourseRegister, JoinCourseInput, StateResult, UserInput},
    users::{User, UserId, Users},
    AppState,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    fs::File,
    hash::{Hash, Hasher},
};
use tide::{prelude::*, Request};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Course {
    name: String,
    users: Vec<UserId>,
}

impl Course {
    fn with_user(name: String, user_id: UserId) -> Self {
        Self {
            name,
            users: vec![user_id],
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
    pub fn get(&self, course_id: String) -> Option<&Course> {
        self.0.get(&course_id)
    }
    pub fn insert(&mut self, course_id: String, course: Course) {
        self.0.insert(course_id, course);
    }
    pub fn new_user(&mut self, user: &User, user_id: UserId) {
        for course in user.courses.iter() {
            self.add_user(user_id, course);
        }
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
    Ok(StateResult::new(true).into())
}
