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
    user_name: String,
    groups: Vec<Group>,
}

#[derive(Serialize)]
pub struct GroupQuery {
    group_name: String,
    users: Vec<User>,
}

// Responses

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupOverviewQueryResult {
    api_version: String,
    groups: Vec<Group>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupQueryResult {
    api_version: String,
    group: GroupQuery,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserQueryResult {
    api_version: String,
    user: UserQuery,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NoResponseResult {
    api_version: String,
    exit_code: i64,
    error: String,
}

pub enum TopLevelResponse {
    GroupOverview(GroupOverviewQueryResult),
    GroupQuery(GroupQueryResult),
    UserQuery(UserQueryResult),
    NoResponse(NoResponseResult)
}