use clap::Parser;
use rust_typer::{Config, VALID_LANGUAGES, run_test};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Language the typing test should use
    #[arg(short, long, default_value_t = String::from("de"))]
    language: String,

    /// Number of words the typing test should use
    #[arg(short, long, default_value_t = 50)]
    words: u32,

    /// Should the test be made of sensible sentences
    #[arg(short, long, default_value_t = false)]
    sensible: bool,
}

fn main() {
    let args = Args::parse();

    if !VALID_LANGUAGES.contains(&args.language.as_str()) {
        eprintln!(
            "Fehler: Ungültige Sprache '{}'. Erlaubt sind: {:?}",
            args.language, VALID_LANGUAGES
        );
        std::process::exit(1);
    }

    if args.words == 0 {
        eprintln!("Fehler: Die Anzahl der Wörter muss größer als 0 sein.");
        std::process::exit(1);
    }

    let config = Config {
        language: args.language,
        words: args.words,
        sensible: args.sensible,
    };

    match run_test(config) {
        Ok(_) => {}
        Err(e) => eprintln!("Ann error ocurred: {}", e),
    };
}
