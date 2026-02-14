pub mod result_eval;
pub mod utils;
pub mod word_generator;

use anyhow::{Result, anyhow};
use std::{
    io::{Stdout, Write, stdout},
    time::SystemTime,
};

pub use utils::{get_ascii_art, get_start_screen, get_test_summary};
pub use word_generator::word_picker::get_words;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute, queue,
    style::{self, Stylize},
    style::{Color, style},
    terminal::{
        self, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode, size,
    },
};

use crate::result_eval::TestResult;

pub const VALID_LANGUAGES: [&str; 2] = ["en", "de"];

#[derive(Debug)]
pub struct Config {
    pub language: String,
    pub words: u32,
    pub sensible: bool,
}

/// Starts the typing test
/// Loads the words and waits for the input of the user
///
/// # Arguments
/// - `config`: config created in main.rs using the user input
///
/// # Returns
/// - `Result<()>`: Result if the test was executed successfully
pub fn run_test(config: Config) -> Result<()> {
    println!("{}", get_ascii_art());

    print!(
        "{}",
        get_start_screen(&config.language, config.words, config.sensible)
    );

    pause!("Press Enter to Start the Test ...");

    let words = get_words(config).unwrap();
    let text = words.join(" ");

    let mut stdout = stdout();
    let mut test_result: TestResult = TestResult::default();

    execute!(stdout, EnterAlternateScreen)?;

    stdout = match write_text_to_terminal(&text, stdout) {
        Ok(stdout) => stdout,
        Err(_) => return Err(anyhow!("Failed to write text to terminal")),
    };

    let start = SystemTime::now();

    let (mut stdout, correct_chars) = match get_user_input(&text, stdout) {
        Ok((stdout, correct_chars)) => (stdout, correct_chars),
        Err(_) => return Err(anyhow!("Unable to get user input")),
    };
    test_result.correct_chars = correct_chars;

    execute!(stdout, LeaveAlternateScreen)?;

    test_result.time = match start.elapsed() {
        Ok(elapsed) => elapsed,
        Err(_) => return Err(anyhow!("Unable to track time")),
    };

    print!("{}", get_test_summary(&mut test_result));
    pause!("Press Enter to get back to your terminal ...");
    Ok(())
}

/// Prints the text the user has to type to the terminal
///
/// # Arguments
/// - `text`: Text to type
/// - `stdout`: stdout object used to print to the terminal
///
/// # Returns
/// - `Result<Stdout>` Returns the provided Stdout object or an Error
fn write_text_to_terminal(text: &str, mut stdout: Stdout) -> Result<Stdout> {
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    queue!(stdout, cursor::MoveTo(0, 0))?;
    queue!(
        stdout,
        style::PrintStyledContent(style(text).with(Color::Reset))
    )?;
    queue!(stdout, cursor::MoveTo(0, 0))?;

    stdout.flush()?;

    Ok(stdout)
}

/// Gets the user input after printing to the terminal
///
/// # Arguments
/// - `text`: Text the user has to type (same text that has been printed to the terminal)
/// - `stdout`: Stdout object used to generate the output
///
/// # Returns
/// - `Result<(Stdout, Vec<bool>)>` Stdout object from the input, Vector of the characters typed
///   correctly. If and error occurs it will be returned
fn get_user_input(text: &str, mut stdout: Stdout) -> Result<(Stdout, Vec<bool>)> {
    enable_raw_mode()?;

    let mut count = 0;
    let mut correct_chars = vec![false; text.len()];

    let max_size = match size() {
        Ok((x, y)) => (x, y),
        Err(_) => return Err(anyhow!("Unable to determine terminal size")),
    };
    loop {
        if let Event::Key(key) = event::read()? {
            // Only when pressing a key not when releasing
            let cursor_position = match cursor::position() {
                Ok((x, y)) => (x, y),
                Err(e) => return Err(anyhow!("Failed to get cursor position, {}", e)),
            };
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char(c) => {
                        if let Ok(is_correct) = check_char(c, count, text) {
                            if is_correct {
                                queue!(stdout, style::PrintStyledContent(c.green()))?;
                                correct_chars[count as usize] = true;
                            } else if let Some(orig_c) = text.chars().nth(count as usize) {
                                queue!(stdout, style::PrintStyledContent(orig_c.red()))?;
                                correct_chars[count as usize] = false;
                            }
                        }
                        count += 1;
                        if count as usize == text.len() {
                            break;
                        }
                    }
                    KeyCode::Backspace => {
                        match cursor_position.0 < 1 {
                            true => {
                                if cursor_position.1 == 0 {
                                    continue;
                                } else {
                                    queue!(
                                        stdout,
                                        cursor::MoveTo(max_size.0, cursor_position.1 - 1)
                                    )?
                                }
                            }
                            false => queue!(stdout, cursor::MoveLeft(1))?,
                        }
                        count -= 1;
                    }
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
            stdout.flush()?;
        }
    }

    disable_raw_mode()?;

    Ok((stdout, correct_chars))
}

/// Checks if a char typed is correct
/// Uses the amount of characters typed and the text to be typed to check if a typed char is
/// correct
///
/// # Arguments
/// - `c`: Character the user typed
/// - `count`: Current amount of chars typed
/// - `text`: Text the user has to type
///
/// # Returns
/// - `Result<bool>` Boolean if the typed char was correct or not.
fn check_char(c: char, count: u32, text: &str) -> Result<bool> {
    if let Some(expected) = text.chars().nth(count as usize) {
        Ok(c == expected)
    } else {
        Err(anyhow!("unable to check char and expected character"))
    }
}
