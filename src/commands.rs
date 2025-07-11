use crate::init_languages;
use std::collections::BTreeMap;

pub struct Command<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub args: Option<BTreeMap<&'a str, init_languages::Language<'a>>>,
}

pub trait CommandExecutor {
    fn execute(&self, argument: &str) -> Result<(), String>;
}

impl CommandExecutor for Command {
    fn execute(&self, argument: &str) -> Result<(), String> {
        match self.name {
            "init" => self.args.as_ref().get(argument)
                .map(|lang| lang.make_project(argument))
                .unwrap_or_else(|| Err(format!("Unsupported language: {}", argument))),
            _ => Err(format!("Command '{}' is not implemented", self.name)),
        }
    }
}

pub const INIT: Command = Command {
    name: "init",
    description: "Initialize the project",
    args: init_languages::SUPPORTED_LANGUAGES.clone(),
};