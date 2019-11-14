use std::string::String;

use serde_derive::Serialize;

#[derive(Serialize, Clone)]
pub struct Group {
    pub name: String,
    pub id: i64,
}

#[derive(Serialize, Clone)]
pub struct User {
    pub name: String,
    pub id: i64,
}

#[derive(Serialize, Clone)]
pub struct UserQuery {
    pub user_name: String,
    pub groups: Vec<Group>,
}

#[derive(Serialize, Clone)]
pub struct GroupQuery {
    pub group_name: String,
    pub users: Vec<User>,
}

// Responses

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroupOverviewQueryResult {
    pub api_version: String,
    pub groups: Vec<Group>,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroupQueryResult {
    pub api_version: String,
    pub group: GroupQuery,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserQueryResult {
    pub api_version: String,
    pub user: UserQuery,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NoResponseResult {
    pub api_version: String,
    pub exit_code: i32,
    pub error: String,
}

#[derive(Clone)]
pub enum TopLevelResponse {
    GroupOverview(GroupOverviewQueryResult),
    GroupQuery(GroupQueryResult),
    UserQuery(UserQueryResult),
    NoResponse(NoResponseResult),
}
