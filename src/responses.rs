use std::fmt::{self, Display, Formatter};
use std::process::exit;
use std::string::String;

use serde_derive::Serialize;
use serde_json::ser;

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
pub struct UserQueryResponse {
    pub user_name: String,
    pub groups: Vec<Group>,
}

#[derive(Serialize, Clone)]
pub struct GroupQueryResponse {
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
    pub group: GroupQueryResponse,
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
    pub user: UserQueryResponse,
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

pub fn output_response(response: TopLevelResponse, is_json: bool) {
    let exit_code = match response.clone() {
        TopLevelResponse::NoResponse(result) => result.exit_code,
        _ => 0,
    };

    match response {
        TopLevelResponse::NoResponse(result) => {
            if is_json {
                let json = ser::to_string(&result).expect("Could not stringify JSON");
                println!("{}", json);
            } else {
                eprintln!("Fatal: {}", result.error);
            }
        }
        TopLevelResponse::GroupOverview(result) => {
            if is_json {
                let json = ser::to_string(&result).expect("Could not stringify JSON");
                println!("{}", json);
            } else {
                println!("{}", result);
            }
        }
        TopLevelResponse::UserQuery(result) => {
            if is_json {
                let json = ser::to_string(&result).expect("Could not stringify JSON");
                println!("{}", json);
            } else {
                println!("{}", result);
            }
        }
        TopLevelResponse::GroupQuery(result) => {
            if is_json {
                let json = ser::to_string(&result).expect("Could not stringify JSON");
                println!("{}", json);
            } else {
                println!("{}", result);
            }
        }
    };

    exit(exit_code);
}
