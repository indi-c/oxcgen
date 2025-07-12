use std::collections::BTreeMap;

use crate::commands;

fn get_command_list<'a>() -> BTreeMap<String, commands::Command<'a>> {
    BTreeMap::from([
        ("init".to_string(), commands::INIT),
    ])
}