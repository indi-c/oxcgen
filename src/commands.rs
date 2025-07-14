use crate::supported_languages;
use crate::language::{ Language, MakeProject };
use std::collections::BTreeMap;


pub struct Command<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub args: [(&'a str, Language<'a>); supported_languages::NUMBER_OF_LANGUAGES],
}

pub trait CommandExecutor {
    fn execute(&self, argument: &str, additional_args: Vec<String>) -> Result<(), String>;
}

impl<'a> CommandExecutor for Command<'a> {
    fn execute(&self, argument: &str, additional_args: Vec<String>) -> Result<(), String> {
        match self.name {
            "init" => {
                let lang_map: BTreeMap<&str, Language> = BTreeMap::from(self.args.clone());
                let project_name: &str = additional_args.get(0).map_or("project", |s| s.as_str());
                lang_map.get(argument)
                    .map(|lang| lang.make_project(project_name))
                    .unwrap_or_else(|| Err(format!("[*] {} not implemented", argument)))
            }
            "help" => Err("Help command is not implemented".to_string()),
            _ => Err(format!("Command '{}' is not valid", self.name)),
        }
    }
}

pub const INIT: Command = Command {
    name: "init",
    description: "Initialize the project",
    args: supported_languages::SUPPORTED_LANGUAGES,
};