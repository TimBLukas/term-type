use std::path::Path;

use super::word_loader::load_words;
use crate::{Config, VALID_LANGUAGES};
use anyhow::{Context, Error, Result, anyhow};

pub fn get_words(config: Config) -> Result<Vec<String>> {
    let filepath = match config.language.as_str() {
        "de" => Path::new("assets")
            .join("books")
            .join("verwandlung-kafka-parsed-german.txt"),
        "en" => Path::new("assets")
            .join("books")
            .join("the-sun-also-rises-parsed-english.txt"),
        _ => {
            return Err(anyhow!(
                "Unable to get words - invalid language ({})",
                config.language
            ));
        }
    };

    println!("The current filepath is {:?}", filepath);
    load_words(config.words, config.sensible, &filepath)
}
