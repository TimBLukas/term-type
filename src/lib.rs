pub mod word_generator;
pub use word_generator::word_picker::get_words;

pub const VALID_LANGUAGES: [&str; 2] = ["en", "de"];

#[derive(Debug)]
pub struct Config {
    pub language: String,
    pub words: u32,
    pub sensible: bool,
}

pub fn run_test(config: Config) {
    println!("Running typing test");

    println!("Args:");
    println!("…\t Language: {}", config.language.as_str());
    println!("…\t Words: {}", config.words);
    println!("…\t Sensible: {}", config.sensible);

    let result = get_words(config).unwrap();

    for word in result {
        println!("{}", word.as_str());
    }
}
