extern crate itertools;

use std::fs;
use std::collections::HashMap;
use itertools::Itertools;

const GROUP_FILE: &'static str = "/etc/group";
const PASSWD_FILE: &'static str = "/etc/passwd";

type StringToStringSet = HashMap<String, Vec<String>>;

fn remove_comment_from_line<'a>(possibly_commented_line: &'a str) -> &str {
    let mut line_split_iter = (*possibly_commented_line).splitn(2, "#").into_iter();
    return line_split_iter.next().expect("Logic error");
}

fn parse_passwd_line<'a>(unparsed_line: &'a str) -> (String, String, String) {
    let split_line = unparsed_line.split("");
    let (user, _, userid, group) = split_line.into_iter().next_tuple().expect("Invalid line");

    return (String::from(user), String::from(userid), String::from(group));
}

fn get_group_data() -> (StringToStringSet, StringToStringSet) {
    let contents = fs::read_to_string(PASSWD_FILE)
        .expect("Something went wrong reading the file");
    
    let lines = contents.lines().into_iter();

    let mut userids_to_users: StringToStringSet = HashMap::new();
    let mut users_to_primarygroups: StringToStringSet = HashMap::new();

    for line in lines {
        let line_cleaned = remove_comment_from_line(line).trim();

        if line_cleaned == "" {
            continue;
        }

        println!("line cleaned: {}", line_cleaned);
    }

    (userids_to_users, users_to_primarygroups)
}

fn main() {
    let (userids_to_users, users_toprimary_groups) = get_group_data();
    // let primary_groups_to_users = get_primary_groups_to_users();

    println!("Hello world");
}