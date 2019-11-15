use std::collections::HashSet;

use crate::errors;

pub use errors::Error;

pub struct TargetObjects {
    pub user_to_list: Option<String>,
    pub group_to_list: Option<String>,
}

#[derive(PartialEq, Eq, Hash)]
pub enum FlagArg {
    JSON,
    HELP,
    USER,
    GROUP,
}

pub fn process_args(
    flag_args: HashSet<FlagArg>,
    pos_args: Vec<String>,
) -> Result<TargetObjects, Box<dyn Error>> {
    if pos_args.len() > 1 {
        return Err(errors::usage_error(
            "Too many positional arguments (expected at most 1)".to_string(),
        ));
    }

    let first_arg = pos_args.iter().next();

    if flag_args.contains(&FlagArg::USER) {
        match first_arg {
            None => {
                return Err(errors::usage_error(
                    "Missing required argument OBJECT".to_string(),
                ));
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
                return Err(errors::usage_error(
                    "Missing required argument OBJECT".to_string(),
                ));
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

pub fn parse_argv_data(
    args: Vec<String>,
) -> Result<(HashSet<FlagArg>, Vec<String>), Box<dyn Error>> {
    let double_hyphen_pos = args.iter().position(|x| x == "--");
    let opt_args = {
        match double_hyphen_pos {
            Some(pos) => &args[..pos],
            None => &args[..],
        }
    };

    let valid_short_flags = vec!["-u", "-g"];
    let valid_long_flags = vec!["--json", "--help", "--user", "--group"];
    let valid_long_flags_iter = valid_long_flags.iter().map(|x| String::from(*x).clone());
    let valid_short_flags_iter = valid_short_flags.iter().map(|x| String::from(*x).clone());

    let matches_long_flag =
        |opt_flag: String| valid_long_flags_iter.clone().any(|valid| opt_flag == valid);
    let matches_short_flag = |opt_flag: String| {
        valid_short_flags_iter
            .clone()
            .any(|valid| opt_flag == valid)
    };

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
