#![feature(try_blocks)]
#![feature(trait_alias)]

mod errors;
mod load;
mod parse;
mod records;
mod responses;
mod shapes;

extern crate itertools;

use std::collections::HashSet;
use std::boxed::Box;
use std::env;
use std::iter::Iterator;
use std::result::Result::{self, Ok, Err};
use std::option::*;
use std::fmt::format;
use std::process::exit;

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

mod error_codes {
    pub const INVALID_USAGE: i32 = 10;
}

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

#[derive(PartialEq, Eq, Hash)]
enum FlagArg {
    JSON,
    HELP,
    USER,
    GROUP
}

fn parse_argv_data(args: Vec<String>) -> Result<
    (HashSet<FlagArg>, Vec<String>),
    Box<dyn Error>
> {
    let double_hyphen_pos = args
        .iter()
        .position(|x| x == "--");
    let opt_args = {
        match double_hyphen_pos {
            Some(pos) => {
                &args[..pos]
            },
            None => {
                &args[..]
            }
        }
    };
    
    let valid_short_flags = vec![
        "-u",
        "-g"
    ];

    let valid_long_flags = vec![
        "--json",
        "--help",
        "--user",
        "--group"
    ];

    let mut valid_long_flags_iter =
        valid_long_flags
            .iter()
            .map(|x| String::from(*x).clone());
    let mut valid_short_flags_iter =
        valid_short_flags
            .iter()
            .map(|x| String::from(*x).clone());

    let mut matches_long_flag = |opt_flag: String| {
        valid_long_flags_iter.any(|valid| opt_flag == valid)
    };
    let mut matches_short_flag = |opt_flag: String| {
        valid_short_flags_iter.any(|valid| opt_flag == valid)
    };

    let mut flag_args: HashSet<FlagArg> = HashSet::new();
    let mut unrecognized_flags: HashSet<String> = HashSet::new();
    let mut positional_args: Vec<String> = vec![];

    let opt_args_as_strings = opt_args.iter().map(|x| x.to_string());

    for opt_arg in opt_args_as_strings {
        if (opt_arg.clone().starts_with("-")) {
            if (matches_long_flag(opt_arg.clone())) {
                if (opt_arg == "--json") {
                    flag_args.insert(FlagArg::JSON);
                } else if (opt_arg == "--help") {
                    flag_args.insert(FlagArg::HELP);
                } else if (opt_arg == "--user") {
                    flag_args.insert(FlagArg::USER);
                } else if (opt_arg == "--group") {
                    flag_args.insert(FlagArg::GROUP);
                } else {
                    return Err(errors::internal_error(
                        format!("unknown long flag {}", opt_arg)
                    ));
                }
            } else if (matches_short_flag(opt_arg.clone())) {
                if (opt_arg == "-u") {
                    flag_args.insert(FlagArg::USER);
                } else if (opt_arg == "-g") {
                    flag_args.insert(FlagArg::GROUP);
                } else {
                    return Err(errors::internal_error(
                        format!("unknown long flag {}", opt_arg)
                    ));
                }
            } else {
                unrecognized_flags.insert(opt_arg);
            }
        } else {
            positional_args.push(opt_arg);
        }
    }

    if let Some(unrecognized_flag) = unrecognized_flags.iter().next() {
        return Err(Box::new(errors::usage_error(
            format!("Unrecognized flag {}", unrecognized_flag)
        )))
    }

    let trailing_pos_args =
        match double_hyphen_pos {
            Some(pos) => {
                &args[pos+1..]
            },
            None => &[]
        }; // TODO tie these in

    return Ok((
        flag_args, positional_args
    ))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let argv_data = parse_argv_data(args);

    match argv_data {
        Err(e) => {
            println!("error");
            // TODO add nice message
            exit(error_codes::INVALID_USAGE)
        },
        Ok((flag_args, pos_args)) => {
            // "explore group memberships"
            //     Arg::from_usage("--json")
            //         .multiple(false)
            //         .takes_value(false)
            //         .required(false)
            //         .help("If set, the output of this command will be a JSON blob"),
            // )
            // .arg(
            //     Arg::with_name("object_name")
            //         .multiple(false)
            //         .takes_value(true)
            //         .required(false)
            //         .help("A user or group (whose groups or users, respectively, are to be listed out)"),
            // )

            // let is_using_json = matches.is_present("json");
            // let mode = match (matches.value_of("mode")) {
            //     Some("-u") | Some("--user") => "user",
            //     Some("-g") | Some("--group") => "group",
            //     Some(&_) | None => "all_groups"
            // };
            // let object_name = matches.value_of("object_name").map(|name| name.to_string());

            // let group_to_list = if (mode == "group") {
            //     object_name.clone()
            // } else {
            //     None
            // };
            // let user_to_list = if (mode == "user") {
            //     object_name.clone()
            // } else {
            //     None
            // };

            // groupls(user_to_list, group_to_list);
        }
    }
}
