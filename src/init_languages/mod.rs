mod lang_struct;

// Re-export the Language struct so it's accessible from parent modules
pub use lang_struct::Language;

use std::collections::BTreeMap;

pub const RUST: Language = Language {
    name: "rust",
    description: "Project template for Rust, includes Cargo.toml, main.rs, and flake.nix",
};

pub fn get_supported_languages() -> BTreeMap<&'static str, Language<'static>> {
    BTreeMap::from([
        ("rust", RUST),
    ])
}