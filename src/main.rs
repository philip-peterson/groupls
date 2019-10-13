#![feature(try_blocks)]
#![feature(trait_alias)]

mod records;
mod errors;
mod shapes;
mod parse;

extern crate itertools;
extern crate clap;

use std::iter::Iterator;
use clap::{Arg, App, SubCommand, ArgGroup};
use std::fs;
use std::result::Result;
use std::boxed::{Box};

use std::path::{Path};

pub use errors::{Error};
pub use shapes::{StringList, IntToStringList, StringToStringList};
pub use records::{GroupEntry, PasswdEntry};

use std::io::{Error as IoError, ErrorKind};

const GROUP_FILE: &'static str = "/etc/group";
const PASSWD_FILE: &'static str = "/etc/passwd";


fn read_users() -> Result<Vec<PasswdEntry>, Box<dyn Error>> {
    let contents = fs::read_to_string(Path::new(PASSWD_FILE)).map_err(
        |e| {
            Box::new(e) as Box<dyn Error>
        }
    )?;
    
    let lines = contents.lines().into_iter();

    let lines_results = lines
        .map(parse::remove_comment_from_line)
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(parse::parse_passwd_line);

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
        .map(parse::remove_comment_from_line)
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(parse::parse_group_line);

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

fn groupls(users_to_list: Vec<&str>, groups_to_list: Vec<&str>) {
    let groups_maybe = read_groups();

    match groups_maybe {
        Ok(groups) => {
            if (groups_to_list.len() == 0 && users_to_list.len() == 0) {
                for group in groups {

                }
            }

            let users = read_users();
        }
        Err(_) => {
            panic!("Could not read groups");
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

    groupls(users_to_list, groups_to_list);
}