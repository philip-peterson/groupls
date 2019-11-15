#![feature(trait_alias)]

mod errors;
mod load;
mod parse_system;
mod records;
mod responses;
mod shapes;
mod args;

extern crate itertools;

use std::boxed::Box;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::iter::Iterator;
use std::process::exit;
use std::result::Result::{self, Err, Ok};

pub use errors::Error;
pub use records::{GroupEntry, PasswdEntry};
pub use responses::{
    GroupOverviewQueryResult, GroupQueryResponse, GroupQueryResult, NoResponseResult,
    TopLevelResponse, User, UserQueryResponse, UserQueryResult,
};
pub use shapes::{IntToStringList, StringList, StringToStringList};
pub use args::{process_args, FlagArg, TargetObjects, parse_argv_data};

use serde_json::ser;

const USAGE_TEXT: &'static str = r#"usage: groupls [--help] [-u | -g | --user | --group]
        [--json] [--] <OBJECT>

`groupls` allows you to explore group permissions.

Supported options:
    -u, --user    Indicates that the OBJECT is the name of a user
    -g, --group   Indicates that the OBJECT is the name of a group
    --help        Displays this help message
    --json        Indicates that the program output should be formatted as JSON.
                  (Note that this is not supported when displaying this help message
                   or error messages.)
    
Three invocation forms are supported:

    groupls
        - prints a list of all groups on this system.
    
    groupls -u alice
        - prints a list of all groups that the user called alice is a member of.
    
    groupls -g admin
        - prints a list of all users that belong to the group called admin.

Untrusted input:

    If invoking `groupls` with untrusted input, be sure to separate the option
    arguments from the positional arguments with a `--` as such:

    groupls -u -- "$USER"
    
    or
    
    groupls -g -- "$GROUP"

    Doing so will ensure that special argument values such as `--help` do not
    interfere with the output formatting.
"#;

mod error_codes {
    pub const INVALID_USAGE: i32 = 10;

    pub const READ_GROUPS_ERROR: i32 = 30;

    pub const READ_USERS_ERROR: i32 = 40;

    pub const GROUP_NOT_FOUND: i32 = 100;
    pub const USER_NOT_FOUND: i32 = 101;
}

fn groupls(target_objects: TargetObjects) -> TopLevelResponse {
    let user_to_list = target_objects.user_to_list;
    let group_to_list = target_objects.group_to_list;

    let groups_raw = load::read_groups();

    let api_version = "1.0".to_string();

    match groups_raw {
        Err(error) => TopLevelResponse::NoResponse(NoResponseResult {
            api_version: api_version,
            error: format!("Could not read groups: {}", error),
            exit_code: error_codes::READ_GROUPS_ERROR,
        }),
        Ok(groups) => {
            match (user_to_list.clone(), group_to_list.clone()) {
                (None, None) => {
                    return TopLevelResponse::GroupOverview(GroupOverviewQueryResult {
                        api_version: api_version,
                        groups: groups
                            .iter()
                            .map(|record| {
                                return responses::Group {
                                    name: record.group.clone(),
                                    id: record.group_id,
                                };
                            })
                            .collect(),
                    });
                }
                _ => {}
            }

            let mut groups_by_id: HashMap<i64, GroupEntry> = HashMap::new();
            for group in groups.iter() {
                groups_by_id.insert(group.group_id, group.clone());
            }

            let users_raw = load::read_users();
            match users_raw {
                Err(error) => {
                    return TopLevelResponse::NoResponse(NoResponseResult {
                        api_version: api_version,
                        error: format!("Could not read users: {}", error),
                        exit_code: error_codes::READ_USERS_ERROR,
                    });
                }
                Ok(users) => {
                    if let Some(user_name) = user_to_list {
                        let found_user = users.iter().find(|u| u.user == user_name);
                        match found_user {
                            Some(found_user) => {
                                let primary_group_id = found_user.primary_group_id;
                                let mut response_groups = vec![];

                                for group in groups {
                                    if group.group_id == primary_group_id
                                        || group.usernames.iter().any(|u| *u == user_name)
                                    {
                                        response_groups.push(responses::Group {
                                            name: group.group,
                                            id: group.group_id,
                                        });
                                    }
                                }

                                return TopLevelResponse::UserQuery(UserQueryResult {
                                    api_version: api_version,
                                    user: UserQueryResponse {
                                        user_name: user_name,
                                        groups: response_groups,
                                    },
                                });
                            }
                            None => {
                                return TopLevelResponse::NoResponse(NoResponseResult {
                                    api_version: api_version,
                                    error: format!("Could not find user: {}", user_name),
                                    exit_code: error_codes::USER_NOT_FOUND,
                                });
                            }
                        }
                    }

                    let group_name = group_to_list.expect("group_to_list was None");
                    let found_group = groups.iter().find(|g| g.group == group_name);
                    match found_group {
                        Some(found_group) => {
                            let mut group_usernames: HashSet<String> = HashSet::new();
                            for username in found_group.usernames.iter() {
                                group_usernames.insert(username.clone());
                            }

                            let mut response_users: Vec<responses::User> = vec![];
                            for user in users {
                                if user.primary_group_id == found_group.group_id
                                    || group_usernames.contains(&user.user)
                                {
                                    response_users.push(responses::User {
                                        name: user.user,
                                        id: user.user_id,
                                    });
                                }
                            }

                            return TopLevelResponse::GroupQuery(responses::GroupQueryResult {
                                api_version: api_version,
                                group: responses::GroupQueryResponse {
                                    group_name: group_name,
                                    users: response_users,
                                },
                            });
                        }
                        None => {
                            return TopLevelResponse::NoResponse(NoResponseResult {
                                api_version: api_version,
                                error: format!("Could not find group: {}", group_name),
                                exit_code: error_codes::GROUP_NOT_FOUND,
                            });
                        }
                    }
                }
            }
        }
    }
}

fn output_response(response: TopLevelResponse, is_json: bool) {
    let exit_code = match response.clone() {
        TopLevelResponse::NoResponse(result) => result.exit_code,
        _ => 0,
    };

    match response {
        TopLevelResponse::NoResponse(result) => {
            eprintln!("Fatal: {}", result.error);
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

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let argv_data = parse_argv_data(args);

    match argv_data {
        Err(e) => {
            eprintln!("Usage error: {}.\n\nFor usage help, try: groupls --help", e);
            exit(error_codes::INVALID_USAGE)
        }
        Ok((flag_args, pos_args)) => {
            if flag_args.contains(&FlagArg::HELP) {
                eprintln!("{}", USAGE_TEXT);
                exit(0);
            }

            let is_json = flag_args.contains(&FlagArg::JSON);
            let processed_args = process_args(flag_args, pos_args);

            match processed_args {
                Ok(target_objects) => {
                    output_response(groupls(target_objects), is_json);
                }
                Err(e) => {
                    eprintln!("Usage error: {}.\n\nFor usage help, try: groupls --help", e);
                    exit(error_codes::INVALID_USAGE)
                }
            };
        }
    }
}
