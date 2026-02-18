use super::word_loader::load_words;
use crate::Config;
use anyhow::{Result, anyhow};
use rand::seq::IteratorRandom;

static GERMAN_BOOKS: [&[u8]; 2] = [
    include_bytes!("../../assets/books/verwandlung-kafka-parsed-german.txt"), // KAFKA_VERWANDLUNG_GERMAN
    include_bytes!("../../assets/books/goethe-faust-parsed-german.txt"),      // GOETHE_FAUST_GERMAN
];

static ENGLISH_BOOKS: [&[u8]; 2] = [
    include_bytes!("../../assets/books/the-sun-also-rises-parsed-english.txt"), // THE_SUN_ALSO_RISES_ENGLISH
    include_bytes!("../../assets/books/illustration-of-today-parsed-english.txt"), // ILLUSTRATION_OF_TODAY_ENGLISH
];

/// Loads the word for a typing test based on the language
///
/// # Arguments
/// - `config`: Config Config object created based on the user input
///
/// # Returns
/// - `Result<Vec<&'a str>>` Vector of string slices (amount = config.words) in the language of the config
pub fn get_words<'a>(config: Config) -> Result<Vec<&'a str>> {
    let book = match config.language.as_str() {
        "de" => get_book(GERMAN_BOOKS),
        "en" => get_book(ENGLISH_BOOKS),
        _ => {
            return Err(anyhow!(
                "Unable to get words - invalid language ({})",
                config.language
            ));
        }
    };

    load_words(config.words, config.sensible, book)
}

fn get_book(input: [&[u8]; 2]) -> &[u8] {
    let mut rng = rand::rng();
    let random_num = (0..input.len()).choose(&mut rng).unwrap_or_default();

    input[random_num]
}
