use std::cmp;

use tide::http::other;

use crate::{
    courses::Course,
    interface::{WeightedUser, WeightedUserList},
    users::User,
};

pub fn find_matches(user: User, other_users: Vec<&User>, course: Course) -> WeightedUserList {
    let mut users_match = vec![];
    let self_desc = user.description.to_array();
    let mut scores = [0; 4];
    let mean_user: isize = user.description.to_array().iter().sum();
    for (i, other_user) in other_users.into_iter().enumerate() {
        let user_score: isize = other_user
            .description
            .to_array()
            .iter()
            .zip(self_desc.iter())
            .map(|(x, y)| cmp::max(x, y))
            .sum();
        let mean_other_user: isize = other_user.description.to_array().iter().sum();

        let weight = (user_score - (user_score - mean_other_user).abs()) as f64
            / user.description.to_array().len() as f64;
        let weighted_user = WeightedUser {
            user: other_user.clone(),
            weight,
        };
        users_match.push(weighted_user);
    }
    WeightedUserList { data: users_match }
}
