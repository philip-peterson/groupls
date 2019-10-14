use std::string::{String};

use serde_derive::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize)]
pub struct Group {
    name: String,
    id: i64,
}

#[derive(Serialize)]
pub struct User {
    name: String,
    id: i64,
}

#[derive(Serialize)]
pub struct UserQuery {
    user_name: String;
    groups: Vec<Group>;
}

#[derive(Serialize)]
pub struct GroupQuery {
    group_name: String;
    users: Vec<User>;
}

// Responses

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupOverview {
    groups: Vec<Group>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectQueryResult {
    group_queries: Vec<GroupQuery>,
    user_queries: Vec<UserQuery>,
}

pub enum TopLevelResponse {
    GroupOverview(GroupOverview),
    ObjectQueryResult(ObjectQueryResult),
}