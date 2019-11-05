use std::string::{String};

use serde_derive::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize)]
pub struct Group {
    pub name: String,
    pub id: i64,
}

#[derive(Serialize)]
pub struct User {
    pub name: String,
    pub id: i64,
}

#[derive(Serialize)]
pub struct UserQuery {
    pub user_name: String,
    pub groups: Vec<Group>,
}

#[derive(Serialize)]
pub struct GroupQuery {
    pub group_name: String,
    pub users: Vec<User>,
}

// Responses

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupOverviewQueryResult {
    pub api_version: String,
    pub groups: Vec<Group>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupQueryResult {
    pub api_version: String,
    pub group: GroupQuery,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserQueryResult {
    pub api_version: String,
    pub user: UserQuery,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NoResponseResult {
    pub api_version: String,
    pub exit_code: i64,
    pub error: String,
}

pub enum TopLevelResponse {
    GroupOverview(GroupOverviewQueryResult),
    GroupQuery(GroupQueryResult),
    UserQuery(UserQueryResult),
    NoResponse(NoResponseResult)
}