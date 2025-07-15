use std::env;

mod commands;
mod command_list;
mod language;
mod supported_languages;

use commands::CommandExecutor;

fn main() {
    let args: Vec<String> = env::args().collect();
    let given_command = args.get(1).map_or("help", |s| s.as_str());
    let commands = command_list::get_command_list();
    let command = &commands.get(given_command);
    let additional_args = args.iter().skip(3).cloned().collect();
    match command.map_or_else(|| Err(format!("[*] unknown command {}", given_command)), |command| command.execute(&args[2], additional_args)) {
        Ok(_) => println!("Command executed successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }
}