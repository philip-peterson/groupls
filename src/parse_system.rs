// Contains logic for parsing system files such as /etc/passwd and /etc/group

pub use crate::errors::{invalid_system_state, missing_field_error, Error};
pub use crate::records::{GroupEntry, PasswdEntry};

pub fn remove_comment_from_line<'a>(possibly_commented_line: &'a str) -> &str {
    let mut line_split_iter = (*possibly_commented_line).splitn(2, "#").into_iter();
    return line_split_iter.next().expect("Logic error");
}

pub fn parse_passwd_line<'a, 'b>(unparsed_line: &'a str) -> Result<PasswdEntry, Box<dyn Error>> {
    let mut split_line = unparsed_line.split(":");

    let username = split_line.next().ok_or(missing_field_error("username"))?;
    let _ = split_line.next(); // skip description
    let userid_raw = split_line.next().ok_or(missing_field_error("user ID"))?;
    let groupid_raw = split_line.next().ok_or(missing_field_error("group ID"))?;

    let userid = String::from(userid_raw);
    let userid_parsed = userid
        .parse::<i64>()
        .map_err(|_| invalid_system_state("user ID number"))?;

    let groupid = String::from(groupid_raw);
    let groupid_parsed = groupid
        .parse::<i64>()
        .map_err(|_| invalid_system_state("group ID number"))?;

    Ok(PasswdEntry {
        user: String::from(username.trim()),
        user_id: userid_parsed,
        primary_group_id: groupid_parsed,
    })
}

pub fn parse_group_line<'a, 'b>(unparsed_line: &'a str) -> Result<GroupEntry, Box<dyn Error>> {
    let mut split_line = unparsed_line.split(":");

    let groupname = split_line.next().ok_or(missing_field_error("group name"))?;
    let _ = split_line.next(); // skip password
    let groupid_raw = split_line.next().ok_or(missing_field_error("group ID"))?;
    let usernames_raw = split_line.next().ok_or(missing_field_error("usernames"))?;

    let groupid = String::from(groupid_raw);
    let groupid_parsed = groupid
        .parse::<i64>()
        .map_err(|_| invalid_system_state("group ID number"))?;

    let usernames = usernames_raw
        .split(",")
        .map(|field| field.trim())
        .filter(|field| !field.is_empty())
        .map(String::from)
        .collect();

    Ok(GroupEntry {
        group: String::from(groupname),
        group_id: groupid_parsed,
        usernames: usernames,
    })
}

