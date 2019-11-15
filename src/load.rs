use crate::errors::Error;
use crate::parse_system::{parse_group_line, parse_passwd_line, remove_comment_from_line};
use crate::records::{GroupEntry, PasswdEntry};
use std::fs;
use std::path::Path;

const GROUP_FILE: &'static str = "/etc/group";
const PASSWD_FILE: &'static str = "/etc/passwd";

pub fn read_users() -> Result<Vec<PasswdEntry>, Box<dyn Error>> {
    let contents =
        fs::read_to_string(Path::new(PASSWD_FILE)).map_err(|e| Box::new(e) as Box<dyn Error>)?;

    let lines = contents.lines().into_iter();

    let lines_results = lines
        .map(remove_comment_from_line)
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(parse_passwd_line);

    let mut line_errors = lines_results
        .clone()
        .filter_map(|result| match result {
            Ok(_) => None,
            Err(e) => Some(e),
        })
        .peekable();

    if let Some(_) = line_errors.peek() {
        for error in line_errors {
            eprintln!("{}", error);
            eprintln!("Unparseable user entry encountered. Skipping...");
        }
    }

    return Ok(lines_results.filter_map(Result::ok).collect());
}

pub fn read_groups<'a>() -> Result<Vec<GroupEntry>, Box<dyn Error>> {
    let contents =
        fs::read_to_string(Path::new(GROUP_FILE)).map_err(|e| Box::new(e) as Box<dyn Error>)?;

    let lines = contents.lines().into_iter();

    let lines_results = lines
        .map(remove_comment_from_line)
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(parse_group_line);

    let mut line_errors = lines_results
        .clone()
        .filter_map(|result| match result {
            Ok(_) => None,
            Err(e) => Some(e),
        })
        .peekable();

    if let Some(_) = line_errors.peek() {
        for error in line_errors {
            eprintln!("{}", error);
            eprintln!("Unparseable group entry encountered. Skipping...");
        }
    }

    let results: std::vec::Vec<GroupEntry> = lines_results.filter_map(Result::ok).collect();

    Ok(results)
}
