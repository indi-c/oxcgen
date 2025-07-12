use crate::supported_languages;
use crate::language::{ Language, MakeProject };
use std::collections::BTreeMap;


pub struct Command<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub args: [(&'a str, Language<'a>); supported_languages::NUMBER_OF_LANGUAGES],
}

pub trait CommandExecutor {
    fn execute(&self, argument: &str) -> Result<(), String>;
}

impl<'a> CommandExecutor for Command<'a> {
    fn execute(&self, argument: &str) -> Result<(), String> {
        match self.name {
            "init" => {
                let lang_map: BTreeMap<&str, Language> = BTreeMap::from(self.args.clone());
                lang_map.get(argument)
                    .map(|lang| lang.make_project("test"))
                    .unwrap_or_else(|| Err(format!("[*] {} not implemented", argument)))
            }
            _ => Err(format!("Command '{}' is not implemented", self.name)),
        }
    }
}

pub const INIT: Command = Command {
    name: "init",
    description: "Initialize the project",
    args: supported_languages::SUPPORTED_LANGUAGES,
};