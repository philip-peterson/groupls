#![feature(try_blocks)]
#![feature(trait_alias)]

mod errors;
mod parse;
mod records;
mod load;
mod shapes;

extern crate clap;
extern crate itertools;

use clap::{App, Arg, ArgGroup, SubCommand};
use std::boxed::Box;
use std::iter::Iterator;
use std::result::Result;

pub use errors::Error;
pub use records::{GroupEntry, PasswdEntry};
pub use shapes::{IntToStringList, StringList, StringToStringList};

use std::io::{Error as IoError, ErrorKind};


fn groupls(users_to_list: Vec<&str>, groups_to_list: Vec<&str>) {
    let groups_maybe = load::read_groups();

    match groups_maybe {
        Ok(groups) => {
            if (groups_to_list.len() == 0 && users_to_list.len() == 0) {
                for group in groups {}
            }

            let users = load::read_users();
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
                .help("If set, the output of this command will be a JSON blob"),
        )
        .arg(
            Arg::with_name("user_name")
                .short("u")
                .long("user")
                .multiple(true)
                .takes_value(true)
                .required(false)
                .help("A user, whose groups are to be listed out"),
        )
        .arg(
            Arg::with_name("group_name")
                .short("g")
                .long("group")
                .multiple(true)
                .takes_value(true)
                .required(false)
                .help("A group, whose users are to be listed out"),
        )
        .after_help(
            "Longer explaination to appear after the options when \
             displaying the help information from --help or -h",
        )
        .get_matches();

    let is_using_json = matches.is_present("json");
    let groups_to_list = matches
        .values_of("group_name")
        .map_or_else(|| Vec::new(), |z| z.collect::<Vec<_>>());
    let users_to_list = matches
        .values_of("user_name")
        .map_or_else(|| Vec::new(), |z| z.collect::<Vec<_>>());

    groupls(users_to_list, groups_to_list);
}
