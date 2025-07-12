use rust_embed::RustEmbed;
use std::fs;
use paste::paste;

// macro: takes a path to a folder and the language name
// embeds the files in the templates folder according to the language name
// then if there is a subfolder, it will embed the files in that subfolder in a new asset

macro_rules! embed_templates {
    // example usage:
    // embed_templates!("rust", "rust");
    // embed_templates!("rust/src", "rust");
    ($path:expr, $lang:expr) => {
        paste! {
            #[derive(RustEmbed)]
            #[folder = [<"templates/" $path>]]
            #[prefix = [<$lang, "/">]]
            pub struct [<$lang "_assets">];
        }
    }
}

embed_templates!("rust", "rust");