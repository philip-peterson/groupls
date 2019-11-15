#![feature(try_blocks)]
#![feature(trait_alias)]

mod errors;
mod load;
mod parse;
mod records;
mod responses;
mod shapes;

extern crate itertools;

use std::boxed::Box;
use std::collections::HashSet;
use std::env;
use std::fmt::format;
use std::iter::Iterator;
use std::option::*;
use std::process::exit;
use std::result::Result::{self, Err, Ok};
use std::collections::HashMap;

pub use errors::Error;
pub use records::{GroupEntry, PasswdEntry};
pub use responses::{
    GroupOverviewQueryResult, GroupQuery, GroupQueryResult, NoResponseResult, TopLevelResponse,
    User, UserQuery, UserQueryResult,
};
pub use shapes::{IntToStringList, StringList, StringToStringList};

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
    
    pub const IO_ERROR: i32 = 20;
    
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
                },
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
                                    if group.group_id == primary_group_id || group.usernames.iter().any(|u| *u == user_name) {
                                        response_groups.push(responses::Group {
                                            name: group.group,
                                            id: group.group_id,
                                        });
                                    }
                                }

                                return TopLevelResponse::UserQuery(UserQueryResult {
                                    api_version: api_version,
                                    user: UserQuery {
                                        user_name: user_name,
                                        groups: response_groups,
                                    },
                                });
                            },
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
                                if user.primary_group_id == found_group.group_id || group_usernames.contains(&user.user) {
                                    response_users.push(responses::User {
                                        name: user.user,
                                        id: user.user_id,
                                    });
                                }
                            }
        
                            return TopLevelResponse::GroupQuery(responses::GroupQueryResult {
                                api_version: api_version,
                                group: responses::GroupQuery {
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

#[derive(PartialEq, Eq, Hash)]
enum FlagArg {
    JSON,
    HELP,
    USER,
    GROUP,
}

struct TargetObjects {
    user_to_list: Option<String>,
    group_to_list: Option<String>,
}

fn process_args(
    flag_args: HashSet<FlagArg>,
    pos_args: Vec<String>,
) -> Result<TargetObjects, Box<dyn Error>> {
    if pos_args.len() > 1 {
        return Err(errors::usage_error("Too many positional arguments (expected at most 1)".to_string()));
    }

    let first_arg = pos_args.iter().next();

    if flag_args.contains(&FlagArg::USER) {
        match first_arg {
            None => {
                return Err(errors::usage_error("Missing required argument OBJECT".to_string()));
            }
            Some(user_name) => {
                return Ok(TargetObjects {
                    user_to_list: Some(user_name.to_string()),
                    group_to_list: None,
                });
            }
        }
    } else if flag_args.contains(&FlagArg::GROUP) {
        match first_arg {
            None => {
                return Err(errors::usage_error("Missing required argument OBJECT".to_string()));
            }
            Some(group_name) => {
                return Ok(TargetObjects {
                    user_to_list: None,
                    group_to_list: Some(group_name.to_string()),
                });
            }
        }
    }

    if let Some(object_name) = first_arg {
        return Err(errors::usage_error(
            format!(
                "Cannot list object of name `{}`; not specified as user or group. Use the `-u` or `-g` flag to specify",
                object_name
            )
        ));
    }

    return Ok(TargetObjects {
        user_to_list: None,
        group_to_list: None,
    });
}

fn parse_argv_data(args: Vec<String>) -> Result<(HashSet<FlagArg>, Vec<String>), Box<dyn Error>> {
    let double_hyphen_pos = args.iter().position(|x| x == "--");
    let opt_args = {
        match double_hyphen_pos {
            Some(pos) => &args[..pos],
            None => &args[..],
        }
    };

    let valid_short_flags = vec!["-u", "-g"];

    let valid_long_flags = vec!["--json", "--help", "--user", "--group"];

    let mut valid_long_flags_iter = valid_long_flags.iter().map(|x| String::from(*x).clone());
    let mut valid_short_flags_iter = valid_short_flags.iter().map(|x| String::from(*x).clone());

    let mut matches_long_flag =
        |opt_flag: String| valid_long_flags_iter.any(|valid| opt_flag == valid);
    let mut matches_short_flag =
        |opt_flag: String| valid_short_flags_iter.any(|valid| opt_flag == valid);

    let mut flag_args: HashSet<FlagArg> = HashSet::new();
    let mut unrecognized_flags: HashSet<String> = HashSet::new();
    let mut positional_args: Vec<String> = vec![];

    let opt_args_as_strings = opt_args.iter().map(|x| x.to_string());

    for opt_arg in opt_args_as_strings {
        if opt_arg.clone().starts_with("-") {
            if matches_long_flag(opt_arg.clone()) {
                if opt_arg == "--json" {
                    flag_args.insert(FlagArg::JSON);
                } else if opt_arg == "--help" {
                    flag_args.insert(FlagArg::HELP);
                } else if opt_arg == "--user" {
                    flag_args.insert(FlagArg::USER);
                } else if opt_arg == "--group" {
                    flag_args.insert(FlagArg::GROUP);
                } else {
                    return Err(errors::internal_error(format!(
                        "unknown long flag {}",
                        opt_arg
                    )));
                }
            } else if matches_short_flag(opt_arg.clone()) {
                if opt_arg == "-u" {
                    flag_args.insert(FlagArg::USER);
                } else if opt_arg == "-g" {
                    flag_args.insert(FlagArg::GROUP);
                } else {
                    return Err(errors::internal_error(format!(
                        "unknown long flag {}",
                        opt_arg
                    )));
                }
            } else {
                unrecognized_flags.insert(opt_arg);
            }
        } else {
            positional_args.push(opt_arg);
        }
    }

    if let Some(unrecognized_flag) = unrecognized_flags.iter().next() {
        if flag_args.contains(&FlagArg::HELP) {
            // As a special case, we ignore unrecognized flags if we can
            // find a --help thrown somewhere in there. This is similar
            // to how some other CLI utilities work.
            return Ok((flag_args, vec![]));
        } else {
            return Err(Box::new(errors::usage_error(format!(
                "Unrecognized flag {}",
                unrecognized_flag
            ))));
        }
    }

    let trailing_pos_args = match double_hyphen_pos {
        Some(pos) => &args[pos + 1..],
        None => &[],
    };

    positional_args.extend_from_slice(trailing_pos_args);

    return Ok((flag_args, positional_args));
}

fn output_response(response: TopLevelResponse, is_json: bool) {
    let exit_code = match response.clone() {
        TopLevelResponse::NoResponse(result) => result.exit_code,
        _ => 0,
    };

    // TODO print output
    match response {
        TopLevelResponse::NoResponse(result) => {
            eprintln!("Fatal: {}", result.error);
        },
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
        _ => {}
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
