use rust_embed::RustEmbed;
use std::fs;

// macro: takes a path to a folder and the language name
// embeds the files in the templates folder according to the language name
// then if there is a subfolder, it will embed the files in that subfolder in a new asset

macro_rules! embed_templates {
    // example usage:
    // embed_templates!("rust", "rust");
    // embed_templates!("rust/src", "rust");
    ($path:expr, $lang:expr) => {
        #[derive(RustEmbed)]
        #[folder = concat!("templates/", $path)]
        #[prefix = concat!($lang, "/")]
        pub struct concat!($lang, "_base");
        // if there is a subfolder, it will embed the files in that subfolder in a new asset
        fs::read_dir(concat!("templates/", $lang))
            .unwrap()
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_dir())
            .for_each(|entry| {
                let subfolder = entry.file_name().to_string_lossy();
                #[derive(RustEmbed)]
                #[folder = concat!(ROOT_PATH, $lang, "/", subfolder)]
                #[prefix = concat!($lang, "/", subfolder, "/")]
                pub struct concat!($lang, "_", subfolder);
            });
    };
}

embed_templates!("rust", "rust");