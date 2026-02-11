pub mod word_generator;
use anyhow::{Error, Result, anyhow};
use std::io::{Stdout, Write, stdout};

pub use word_generator::word_picker::get_words;

use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    event::{self, Event, KeyCode, KeyEventKind, read},
    execute, queue,
    style::{self, Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor, Stylize},
    terminal::{
        self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode, size,
    },
};

pub const VALID_LANGUAGES: [&str; 2] = ["en", "de"];

#[derive(Debug)]
pub struct Config {
    pub language: String,
    pub words: u32,
    pub sensible: bool,
}

pub fn run_test(config: Config) -> Result<()> {
    println!("Running typing test");

    println!("Args:");
    println!("…\t Language: {}", config.language.as_str());
    println!("…\t Words: {}", config.words);
    println!("…\t Sensible: {}", config.sensible);

    let words = get_words(config).unwrap();

    let text = words.join(" ");

    let mut stdout = stdout();

    execute!(stdout, EnterAlternateScreen)?;

    stdout = match write_text_to_terminal(&text, stdout) {
        Ok(stdout) => stdout,
        Err(e) => return Err(anyhow!("Failed to write text to terminal")),
    };

    stdout = match get_user_input(&text, stdout) {
        Ok(stdout) => stdout,
        Err(e) => return Err(anyhow!("Unable to get user input")),
    };

    execute!(stdout, LeaveAlternateScreen)?;

    Ok(())
}

fn write_text_to_terminal(text: &str, mut stdout: Stdout) -> Result<Stdout> {
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    queue!(stdout, cursor::MoveTo(0, 0))?;
    queue!(
        stdout,
        style::PrintStyledContent(text.white().on_dark_grey())
    )?;
    queue!(stdout, cursor::MoveTo(0, 0))?;

    stdout.flush()?;

    Ok(stdout)
}

fn get_user_input(text: &str, mut stdout: Stdout) -> Result<Stdout> {
    enable_raw_mode()?;

    let mut count = 0;
    let MAX_SIZE = match size() {
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
                                queue!(
                                    stdout,
                                    style::PrintStyledContent(c.green().on_dark_grey())
                                )?;
                            } else if let Some(orig_c) = text.chars().nth(count as usize) {
                                queue!(
                                    stdout,
                                    style::PrintStyledContent(orig_c.red().on_dark_grey())
                                )?;
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
                                        cursor::MoveTo(MAX_SIZE.0, cursor_position.1 - 1)
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

    Ok(stdout)
}

fn check_char(c: char, count: u32, text: &str) -> Result<bool> {
    if let Some(expected) = text.chars().nth(count as usize) {
        Ok(c == expected)
    } else {
        Err(anyhow!("unable to check char and expected character"))
    }
}
