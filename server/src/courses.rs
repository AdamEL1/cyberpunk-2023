use crate::{
    interface::CourseRegister,
    users::{UserId, Users},
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
    // pub fn add_user(&mut self, users: &Users) {
    //     for (user_id, user) in users.iter() {
    //         for course in user.
    //     }
    // }
}

impl From<CourseRegister> for Course {
    fn from(value: CourseRegister) -> Self {
        Self {
            name: value.name,
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
    write.insert(course.name.clone(), course.into());
    Ok("Ok\n".into())
}
