use super::word_loader::load_words;
use crate::Config;
use anyhow::{Result, anyhow};

static GERMAN_BOOK: &[u8] =
    include_bytes!("../../assets/books/verwandlung-kafka-parsed-german.txt");

static ENGLISH_BOOK: &[u8] =
    include_bytes!("../../assets/books/the-sun-also-rises-parsed-english.txt");

/// Loads the word for a typing test based on the language
///
/// # Arguments
/// - `config`: Config Config object created based on the user input
///
/// # Returns
/// - `Result<Vec<&'a str>>` Vector of string slices (amount = config.words) in the language of the config
pub fn get_words<'a>(config: Config) -> Result<Vec<&'a str>> {
    let book = match config.language.as_str() {
        "de" => GERMAN_BOOK,
        "en" => ENGLISH_BOOK,
        _ => {
            return Err(anyhow!(
                "Unable to get words - invalid language ({})",
                config.language
            ));
        }
    };

    load_words(config.words, config.sensible, book)
}
