// Entry from /etc/passwd representing a user
pub struct PasswdEntry {
    pub user: String,
    pub user_id: i64,
    pub primary_group_id: i64,
}

// Entry from /etc/group representing a group
pub struct GroupEntry {
    pub group: String,
    pub group_id: i64,
    pub usernames: Vec<String>,
}
