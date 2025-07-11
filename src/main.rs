use std::env;
use std::fs;
use std::hash::Hash;
use std::path::PathBuf;
use std::string;
use rust_embed::RustEmbed;
use std::collections::BTreeMap;

mod commands;
mod command_list;
mod init_languages;

fn main() {
    let args: Vec<String> = env::args().collect();
    
}