#![feature(try_blocks)]
#![feature(trait_alias)]

mod records;
mod errors;
mod shapes;

extern crate itertools;
extern crate clap;

use std::iter::Iterator;
use clap::{Arg, App, SubCommand, ArgGroup};
use std::fs;
use std::result::Result;
use std::boxed::{Box};

use std::path::{Path};

pub use errors::{missing_field_error, invalid_system_state, Error};
pub use shapes::{StringList, IntToStringList, StringToStringList};
pub use records::{GroupEntry, PasswdEntry};

use std::io::{Error as IoError, ErrorKind};

const GROUP_FILE: &'static str = "/etc/group";
const PASSWD_FILE: &'static str = "/etc/passwd";

fn remove_comment_from_line<'a>(possibly_commented_line: &'a str) -> &str {
    let mut line_split_iter = (*possibly_commented_line).splitn(2, "#").into_iter();
    return line_split_iter.next().expect("Logic error");
}

fn parse_passwd_line<'a, 'b>(unparsed_line: &'a str) -> Result<PasswdEntry, Box<dyn Error>> {
    let mut split_line = unparsed_line.split(":");

    let username = split_line.next().ok_or( missing_field_error("username")  )?;
    let _ = split_line.next(); // skip description
    let userid_raw = split_line.next().ok_or(missing_field_error("user ID") )?;
    let groupid_raw = split_line.next().ok_or(missing_field_error("group ID") )?;

    let userid = String::from(userid_raw);
    let userid_parsed = userid.parse::<i64>().map_err(|_| invalid_system_state("user ID number") )?;

    let groupid = String::from(userid_raw);
    let groupid_parsed = userid.parse::<i64>().map_err(|_| invalid_system_state("group ID number") )?;

    Ok(PasswdEntry{
        user: String::from(username.trim()),
        user_id: userid_parsed,
        primary_group_id: groupid_parsed,
    })
}

fn parse_group_line<'a, 'b>(unparsed_line: &'a str) -> Result<GroupEntry, Box<dyn Error>> {
    let mut split_line = unparsed_line.split(":");

    let groupname = split_line.next().ok_or(missing_field_error("group name"))?;
    let _ = split_line.next(); // skip password
    let groupid_raw = split_line.next().ok_or(missing_field_error("group ID"))?;
    let usernames_raw = split_line.next().ok_or(missing_field_error("usernames"))?;

    let groupid = String::from(groupid_raw);
    let groupid_parsed = groupid.parse::<i64>().map_err(|_| invalid_system_state("group ID number"))?;

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

fn read_users() -> Result<Vec<PasswdEntry>, Box<dyn Error>> {
    let contents = fs::read_to_string(Path::new(PASSWD_FILE)).map_err(
        |e| {
            Box::new(e) as Box<dyn Error>
        }
    )?;
    
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
            eprintln!("{}", error);
            eprintln!("Unparseable user entry encountered. Skipping...");
        }
    }

    return Ok(lines_results.filter_map(Result::ok).collect());
}

fn read_groups<'a>() -> Result<Vec<GroupEntry>, Box<dyn Error>> {
    let contents = fs::read_to_string(Path::new(GROUP_FILE)).map_err(
        |e| {
            Box::new(e) as Box<dyn Error>
        }
    )?;

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
            eprintln!("{}", error);
            eprintln!("Unparseable group entry encountered. Skipping...");
        }
    }

    let results: std::vec::Vec<records::GroupEntry> = lines_results.filter_map(Result::ok).collect();
    return Ok(results);
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
                .help("If set, the output of this command will be a JSON blob")
        )
        .arg(
            Arg::with_name("user_name")
                .short("u")
                .long("user")
                .multiple(true)
                .takes_value(true)
                .required(false)
                .help("A user, whose groups are to be listed out")
        )
        .arg(
            Arg::with_name("group_name")
                .short("g")
                .long("group")
                .multiple(true)
                .takes_value(true)
                .required(false)
                .help("A group, whose users are to be listed out")
        )
        .after_help("Longer explaination to appear after the options when \
            displaying the help information from --help or -h")
        .get_matches();

    let is_using_json = matches.is_present("json");
    let groups_to_list = matches.values_of("group_name").map_or_else(|| Vec::new(), |z| z.collect::<Vec<_>>());
    let users_to_list = matches.values_of("user_name").map_or_else(|| Vec::new(), |z| z.collect::<Vec<_>>());

    let groups = read_groups();

    if (groups_to_list.len() == 0 && users_to_list.len() == 0) {
        println!("There are 3 groups that exist");
    }

    let users = read_users();
}