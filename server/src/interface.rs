use tide::{prelude::*, Request};

use crate::{courses::Course, users::Description};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CourseRegister {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserRegister {
    pub name: String,
    pub password: String,
    pub school: String,
    pub courses: Vec<CourseRegister>,
    pub email: String,
    pub description: Description,
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
