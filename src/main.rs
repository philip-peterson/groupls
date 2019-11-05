#![feature(try_blocks)]
#![feature(trait_alias)]

mod errors;
mod load;
mod parse;
mod records;
mod responses;
mod shapes;

extern crate clap;
extern crate itertools;

use clap::{App, Arg, ArgGroup, SubCommand};
use std::boxed::Box;
use std::iter::Iterator;
use std::result::Result::{self, Ok, Err};
use std::option::*;

pub use errors::Error;
pub use records::{GroupEntry, PasswdEntry};
pub use shapes::{IntToStringList, StringList, StringToStringList};
pub use responses::{
    NoResponseResult,
    UserQueryResult,
    GroupQueryResult,
    TopLevelResponse,
    GroupQuery,
    UserQuery,
    GroupOverviewQueryResult,
    User,
};

use std::io::{Error as IoError, ErrorKind};

fn groupls(user_to_list: Option<String>, group_to_list: Option<String>) -> TopLevelResponse {
    let groups_raw = load::read_groups();

    match groups_raw {
        Err(error) => TopLevelResponse::NoResponse(
            NoResponseResult {
                api_version: "1.0".to_string(),
                error: format!("{}", error),
                exit_code: 10
            }
        ),
        Ok(groups) => {
            if let None = user_to_list {
                if let None = group_to_list {
                    return TopLevelResponse::GroupOverview(
                        GroupOverviewQueryResult {
                            api_version: "1.0".to_string(),
                            groups: groups.iter().map(|record| {
                                return responses::Group {
                                    name: record.group.clone(),
                                    id: record.group_id,
                                }
                            }).collect()
                        }
                    );
                }
            }

            let users_raw = load::read_users();
            match users_raw {
                Err(error) => {
                    return TopLevelResponse::NoResponse(
                        NoResponseResult {
                            api_version: "1.0".to_string(),
                            error: format!("{}", error),
                            exit_code: 20
                        }
                    );
                },
                Ok(users) => {
                    if let Some(_) = user_to_list {
                        return TopLevelResponse::UserQuery(
                            UserQueryResult {
                                api_version: "1.0".to_string(),
                                user: UserQuery {
                                    user_name: "foobar".to_string(),
                                    groups: vec![responses::Group {
                                        name: "foo".to_string(),
                                        id: 1
                                    }]
                                }
                            }
                        );
                    }

                    group_to_list.unwrap();

                    return TopLevelResponse::GroupQuery(
                        responses::GroupQueryResult {
                            api_version: "1.0".to_string(),
                            group: responses::GroupQuery {
                                group_name: "foobar".to_string(),
                                users: vec![responses::User {
                                    name: "foo".to_string(),
                                    id: 1
                                }]
                            }
                        }
                    );
                }
            }
        }
    }
}

fn main() {
    let matches = App::new("groupls")
        .author("pc.peterso@gmail.com")
        .version("1.0.0")
        .about("explore group memberships")
        .arg(
            Arg::from_usage("--json")
                .multiple(false)
                .takes_value(false)
                .required(false)
                .help("If set, the output of this command will be a JSON blob"),
        )
        .arg(
            Arg::with_name("mode")
                .short("u")
                .short("g")
                .long("user")
                .long("group")
                .multiple(false)
                .required(false)
                .help("If set, the output of this command will be a JSON blob"),
        )
        .arg(
            Arg::with_name("object_name")
                .multiple(false)
                .takes_value(true)
                .required(false)
                .help("A user or group (whose groups or users, respectively, are to be listed out)"),
        )
        .after_help(
            "Longer explaination to appear after the options when \
             displaying the help information from --help or -h",
        )
        .get_matches();

    let is_using_json = matches.is_present("json");
    let mode = match (matches.value_of("mode")) {
        Some("u") | Some("user") => "user",
        Some("g") | Some("group") => "group",
        Some(&_) | None => "all_groups"
    };
    let object_name = matches.value_of("object_name").map(|name| name.to_string());

    let group_to_list = if (mode == "group") {
        object_name.clone()
    } else {
        None
    };
    let user_to_list = if (mode == "user") {
        object_name.clone()
    } else {
        None
    };

    groupls(user_to_list, group_to_list);
}
