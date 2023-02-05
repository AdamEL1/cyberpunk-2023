use std::cmp;

use crate::{
    interface::{WeightedUser, WeightedUserList},
    users::User,
};

pub fn find_matches(user: User, other_users: Vec<User>) -> WeightedUserList {
    let mut users_match = vec![];
    let self_desc = user.description.to_array();
    let mean_user: isize = user.description.to_array().iter().sum();
    for other_user in other_users {
        let user_score: isize = other_user
            .description
            .to_array()
            .iter()
            .zip(self_desc.iter())
            .map(|(x, y)| cmp::max(x, y))
            .sum();
        let mean_other_user: isize = other_user.description.to_array().iter().sum();

        let weight = (user_score - (mean_user - mean_other_user).abs()) as f64
            / user.description.to_array().len() as f64;
        let weighted_user = WeightedUser {
            user: other_user.clone(),
            weight,
        };
        users_match.push(weighted_user);
    }
    WeightedUserList { data: users_match }
}
