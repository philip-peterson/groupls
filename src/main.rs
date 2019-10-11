#![feature(try_blocks)]

mod records;
mod errors;
mod shapes;

extern crate itertools;
extern crate clap;

use clap::{Arg, App};
use std::fs;
use std::result::Result;
use std::num::{ParseIntError};

use std::io::{Error, ErrorKind};
use std::path::{Path};

pub use errors::{missing_field_error};
pub use shapes::{StringList, IntToStringList, StringToStringList};
pub use records::{GroupEntry, PasswdEntry};

const GROUP_FILE: &'static str = "/etc/group";
const PASSWD_FILE: &'static str = "/etc/passwd";

fn remove_comment_from_line<'a>(possibly_commented_line: &'a str) -> &str {
    let mut line_split_iter = (*possibly_commented_line).splitn(2, "#").into_iter();
    return line_split_iter.next().expect("Logic error");
}

fn parse_passwd_line<'a>(unparsed_line: &'a str) -> Result<PasswdEntry, Error> {
    let mut split_line = unparsed_line.split(":");

    let username = split_line.next()
        .ok_or(
            missing_field_error("username")
        )?;
    let _ = split_line.next(); // skip description
    let userid_raw = split_line.next().expect("Invalid line (missing field: user ID)");
    let groupid_raw = split_line.next().expect("Invalid line (missing field: group ID)");

    let userid = String::from(userid_raw);
    let userid_parsed = userid.parse::<i64>().expect("Invalid user ID");

    let groupid = String::from(userid_raw);
    let groupid_parsed = userid.parse::<i64>().expect("Invalid group ID");

    Ok(PasswdEntry{
        user: String::from(username.trim()),
        user_id: userid_parsed,
        primary_group_id: groupid_parsed,
    })
}

fn parse_group_line<'a>(unparsed_line: &'a str) -> Result<GroupEntry, ParseIntError> {
    let mut split_line = unparsed_line.split(":");

    let groupname = split_line.next().expect("Invalid line (missing field: group name)");
    let _ = split_line.next(); // skip password
    let groupid_raw = split_line.next().expect("Invalid line (missing field: group ID)");
    let usernames_raw = split_line.next().expect("Invalid line (missing field: usernames)");

    let groupid = String::from(groupid_raw);
    let groupid_parsed = groupid.parse::<i64>().expect("Invalid group ID");

    let usernames = usernames_raw
        .split(",")
        .map(|field| field.trim())
        .filter(|field| !field.is_empty())
        .map(String::from)
        .collect();
    
    Ok(GroupEntry{
        group: String::from(groupname),
        group_id: groupid_parsed,
        usernames: usernames
    })
}

fn read_users() -> Vec<PasswdEntry> {
    let contents = fs::read_to_string(Path::new(PASSWD_FILE))
        .expect("Something went wrong reading the file");
    
    let lines = contents.lines().into_iter();

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
            println!("Unparseable user entry encountered. Skipping...");
        }
    }

    return lines_results.filter_map(Result::ok).collect();
}

fn read_groups() -> Vec<GroupEntry> {
    let contents = fs::read_to_string(Path::new(GROUP_FILE))
        .expect("Something went wrong reading the file");
    
    let lines = contents.lines().into_iter();

    let lines_results = lines
        .map(remove_comment_from_line)
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(parse_group_line);

    let mut line_errors = lines_results.clone().filter_map(|result| {
        match result {
            Ok(_) => None,
            Err(e) => Some(e)
        }
    }).peekable();

    if let Some(_) = line_errors.peek() {
        for error in line_errors {
            println!("Unparseable group entry encountered. Skipping...");
        }
    }

    return lines_results.filter_map(Result::ok).collect();
}

fn main() {
    let users = read_users();
    let groups = read_groups();
}