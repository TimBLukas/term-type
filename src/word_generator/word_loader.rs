use anyhow::{Result, anyhow};
use rand::RngExt;
use rand::seq::IteratorRandom;

/// Wrapper function that differentiates between sensible and random words based on the sensible
/// flag of the config Object
///
/// # Args
///     amount: u32 Amount of words in the typing test
///     sensible: bool Should the words / sequence of words should be sensible
///     book: &[u8] content of the book the words should be loaded from
/// # Returns
///     Result<Vec<&str>> Vector of string slices representing the words for the typing test
pub fn load_words(amount: u32, sensible: bool, book: &[u8]) -> Result<Vec<&str>> {
    match sensible {
        true => load_words_sensible(amount, book),
        false => load_words_random(amount, book),
    }
}

fn load_words_sensible(amount: u32, book: &[u8]) -> Result<Vec<&str>> {
    let file_content = match get_book_content(book) {
        Ok(content) => content,
        Err(e) => return Err(anyhow!("Error loading file content: {}", e)),
    };

    let amount_as_usize = amount as usize;
    let start_idx = get_start_idx(file_content.len(), amount_as_usize);
    let end_idx = start_idx + amount_as_usize;

    let sub_vec = file_content[start_idx..end_idx].to_vec();
    Ok(sub_vec)
}

fn load_words_random(amount: u32, book: &[u8]) -> Result<Vec<&str>> {
    let book_content = match get_book_content(book) {
        Ok(content) => content,
        Err(e) => return Err(anyhow!("Error loading file content: {}", e)),
    };

    let indices = get_random_indices(book_content.len(), amount as usize);

    let mut result = Vec::new();
    for idx in indices {
        result.push(book_content[idx]);
    }

    Ok(result)
}

fn get_book_content(book: &[u8]) -> Result<Vec<&str>> {
    let book_content = match std::str::from_utf8(book) {
        Ok(book_content) => book_content,
        Err(_) => return Err(anyhow!("Unable to access book content")),
    };

    let book_content_splitted = book_content.split('\n').collect();

    Ok(book_content_splitted)
}

fn get_start_idx(content_len: usize, amount: usize) -> usize {
    let mut rng = rand::rng();
    (0..(content_len - amount))
        .choose(&mut rng)
        .unwrap_or_default()
}

fn get_random_indices(content_len: usize, amount: usize) -> Vec<usize> {
    let mut rng = rand::rng();

    let vals: Vec<usize> = (0..amount)
        .map(|_| rng.random_range(0..content_len))
        .collect();

    vals
}
