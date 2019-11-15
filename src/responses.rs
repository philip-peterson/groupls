use std::fmt::{self, Display, Formatter};
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

impl Display for GroupOverviewQueryResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, group) in self.groups.iter().enumerate() {
            if i != 0 {
                write!(f, "\n")?;
            }
            write!(f, "{}", group.name)?;
        }
        write!(f, "")
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroupQueryResult {
    pub api_version: String,

    // TODO rename GroupQuery and similar types, as these names don't make sense
    pub group: GroupQuery,
}

impl Display for GroupQueryResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, user) in self.group.users.iter().enumerate() {
            if i != 0 {
                write!(f, "\n")?;
            }
            write!(f, "{}", user.name)?;
        }
        write!(f, "")
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserQueryResult {
    pub api_version: String,
    pub user: UserQuery,
}

impl Display for UserQueryResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, group) in self.user.groups.iter().enumerate() {
            if i != 0 {
                write!(f, "\n")?;
            }
            write!(f, "{}", group.name)?;
        }
        write!(f, "")
    }
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
