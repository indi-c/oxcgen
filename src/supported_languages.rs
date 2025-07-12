use crate::language::Language;

pub const RUST: Language = Language {
    name: "rust",
    description: "Project template for Rust, includes Cargo.toml, main.rs, and flake.nix",
};

pub const NUMBER_OF_LANGUAGES: usize = 1;

pub const SUPPORTED_LANGUAGES: [(&str, Language); NUMBER_OF_LANGUAGES] = [
    ("rust", RUST),
];