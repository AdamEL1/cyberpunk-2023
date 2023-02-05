use tide::{prelude::*, Request};

use crate::courses::Course;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CourseRegister {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserRegister {
    pub name: String,
    pub password: String,
    pub email: String,
    pub university: String,
    pub courses: Vec<CourseRegister>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserInput {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AddCourseInput {
    pub name: String,
    pub password: String,
    pub course: CourseRegister,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserRegisterResult {
    pub state: bool,
}
