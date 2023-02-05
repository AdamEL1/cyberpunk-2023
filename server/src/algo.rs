use std::cmp;

use crate::{courses::Course, users::User};

pub struct MatchInput {
    user: User,
    course: String,
}

pub struct UserWeight {
    user: User,
    weight: f64,
}

// pub fn find_matches(user: User, other_users: Vec<User>, course: Course) -> Vec<UserWeight> {
//     let mut users_match = vec![UserWeight {
//         user: user.clone(),
//         weight: 0.0,
//     }];
//     let self_desc = user.description.to_array();
//     let mut scores = [0; 4];
//     for (i, other_user) in other_users.iter().enumerate() {
//         let user_score: isize = other_user
//             .description
//             .to_array()
//             .iter()
//             .zip(self_desc.iter())
//             .map(|(x, y)| cmp::max(x, y))
//             .sum();
//         scores[i] = user_score;
//     }
//     let max = *score.iter().max().unwrap() as f64;
//     for score in 0..scores.len() {}
//     users_match
// }
