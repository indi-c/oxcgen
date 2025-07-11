use std::collections::BTreeMap;

use crate::commands;

fn get_command_list() -> BTreeMap<&str, commands::Command> {
    BTreeMap::from([
        ("init", commands::INIT),
    ])
}