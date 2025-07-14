use crate::language::Language;
use paste::paste;
use rust_embed::RustEmbed;

pub const NUMBER_OF_LANGUAGES: usize = 1;

macro_rules! embed_templates {
    // example usage:
    // embed_templates!("rust", "rust");
    ($path:expr, $lang:expr) => {
        assets_macro::make_asset!($lang);
    }
}

macro_rules! define_languages {
    ($($lang:ident),*) => {
        $(
            paste!
            {
                embed_templates!($lang, $lang);
            }
        )*

        #[derive(Debug, Clone, Copy)]
        pub enum Assets {
            $(
                $lang,
            )*
        }

        impl Assets {
            pub fn get(&self, path: &str) -> Option<rust_embed::EmbeddedFile> {
                match self {
                    $(
                        Assets::$lang => {
                            paste! {
                                [<$lang Assets>]::get(path)
                            }
                        },
                        _ => None,
                    )*
                }
            }
        }

        pub fn from_str(name: &str) -> Option<Assets> {
            match name {
                $(
                    stringify!($lang) => Some(Assets::$lang),
                )*
                _ => None,
            }
        }
        
        $(
            paste! {
                pub const [<$lang:upper>]: Language = Language {
                    name: stringify!($lang),
                };
            }
        )*

        pub const SUPPORTED_LANGUAGES: [(&str, Language); NUMBER_OF_LANGUAGES] = [
            $(
                paste! {
                    (stringify!($lang), [<$lang:upper>])
                },
            )*
        ];
    };
}

define_languages!(Rust);