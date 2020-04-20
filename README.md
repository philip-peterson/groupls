# GroupLS

## Overview

`groupls` is a Unix command-line utility to explore group memberships. Ever wanted to find
a list of all users belonging to a group? GroupLS can do that for you.

## Features

### Listings
With this tool, you can see all the users belonging to a group, all the groups a user belongs to,
or just a list of all the groups on the system. Future plans include a "tree view" where you can
view a more detailed overview detailing both the groups available as well as the users in each group.

### JSON Output
You can easily output to JSON, ready to be parsed by another utility such as `jq`. Just add the `--json`
flag to your command.

## Language
This program was written entirely in Rust. The intention was to serve as a personal project for
yours truly to learn the language. If you have feedback or suggestions on how to do things better,
I would love to hear! Just shoot me an email, or [open an issue](https://github.com/philip-peterson/groupls/issues/new).

## Installation

On BSD, run `./install-bsd.sh`

On Linux, run `./install-linux.sh`

## Examples

```shell
$ # simple zero-arg usage -- lists all groups
$ groupls
nobody
nogroup
wheel
daemon
sys
tty
operator
mail
bin
owner
everyone
_taskgated
group
staff
_www
```

```shell
$ # list all users belonging to a group
$ groupls -g _www
_www
_teamsserver
_devicemgr
```

```shell
$ # list all groups that the user _www is in
$ groupls -u _www
_www
```

```shell
$ # list all the groups that user _teamsserver is in
$ groupls -u _teamsserver
mail
_www
_calendar
_teamsserver
_odchpass
_postgres
_webauthserver
```

```shell
$ # Add --json for easy JSON output!
$ groupls --json -u _teamsserver
{"apiVersion":"1.0","user":{"user_name":"_teamsserver","groups":[{"name":"mail","id":6},{"name":"_www","id":70},{"name":"_calendar","id":93},{"name":"_teamsserver","id":94},{"name":"_odchpass","id":209},{"name":"_postgres","id":216},{"name":"_webauthserver","id":221}]}}
```
