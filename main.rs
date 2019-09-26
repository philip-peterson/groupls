extern crate itertools;

use std::fs;
use std::collections::HashMap;
use itertools::Itertools;
use std::result::Result;
use std::any::Any;
use std::num::{ParseIntError};

use std::io;
use std::io::prelude::*; 

const GROUP_FILE: &'static str = "/etc/group";
const PASSWD_FILE: &'static str = "/etc/passwd";

type StringToStringSet = HashMap<String, Vec<String>>;

// Entry from /etc/passwd representing a user
struct PasswdEntry {
    user: String,
    user_id: i64,
    group: String,
}

fn remove_comment_from_line<'a>(possibly_commented_line: &'a str) -> &str {
    let mut line_split_iter = (*possibly_commented_line).splitn(2, "#").into_iter();
    return line_split_iter.next().expect("Logic error");
}

fn parse_passwd_line<'a>(unparsed_line: &'a str) -> Result<PasswdEntry, ParseIntError> {
    let mut split_line = unparsed_line.split(":");

    let user = split_line.next().expect("Invalid line (missing field: user)");
    let _ = split_line.next();
    let userid_raw = split_line.next().expect("Invalid line  (missing field: user ID)");
    let group = split_line.next().expect("Invalid line (missing field: group)");

    let userid = String::from(userid_raw);
    let userid_parsed = userid.parse::<i64>()?;

    Ok(PasswdEntry{
        user: String::from(user),
        user_id: userid_parsed,
        group: String::from(group),
    })
}

fn get_group_data() -> (StringToStringSet, StringToStringSet) {
    let contents = fs::read_to_string(PASSWD_FILE)
        .expect("Something went wrong reading the file");
    
    let lines = contents.lines().into_iter();

    let userids_to_users: StringToStringSet = HashMap::new();
    let users_to_primarygroups: StringToStringSet = HashMap::new();

    let lines_results = lines
        .map(remove_comment_from_line)
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(parse_passwd_line);
    
    let mut line_errors = lines_results.clone().filter_map(|result| {
        match result {
            Ok(_) => None,
            Err(e) => Some(e)
        }
    }).peekable();

    if let Some(_) = line_errors.peek() {
        for error in line_errors {
            println!("Unparseable /etc/passwd entry encountered. Skipping...");
        }
    }

    for line_result in lines_results.filter_map(Result::ok) {
        println!("user = {}", line_result.user);
    }

    (userids_to_users, users_to_primarygroups)
}

fn main() {
    let (userids_to_users, users_to_primarygroups) = get_group_data();
    // let primary_groups_to_users = get_primary_groups_to_users();
}