use anyhow::{Result, anyhow};
use rand::seq::IteratorRandom;
use rand::{Rng, RngExt};
use std::{fs::read_to_string, path::PathBuf};

pub fn load_words(amount: u32, sensible: bool, filepath: &PathBuf) -> Result<Vec<String>> {
    match sensible {
        true => load_words_sensible(amount, filepath),
        false => load_words_random(amount, filepath),
    }
}

fn load_words_sensible(amount: u32, filepath: &PathBuf) -> Result<Vec<String>> {
    let file_content = match get_file_content(&filepath) {
        Ok(content) => content,
        Err(e) => return Err(anyhow!("Error loading file content: {}", e.to_string())),
    };

    let amount_as_usize = amount as usize;
    let start_idx = get_start_idx(file_content.len(), amount_as_usize);
    let end_idx = start_idx + amount_as_usize;

    let sub_vec = file_content[start_idx..end_idx].to_vec();
    Ok(sub_vec)
}

fn load_words_random(amount: u32, filepath: &PathBuf) -> Result<Vec<String>> {
    let file_content = match get_file_content(&filepath) {
        Ok(content) => content,
        Err(e) => return Err(anyhow!("Error loading file content: {}", e.to_string())),
    };

    let indices = get_random_indices(file_content.len(), amount as usize);

    let mut result = Vec::new();
    for idx in indices {
        result.push(file_content[idx].clone());
    }

    Ok(result)
}

fn get_file_content(filepath: &PathBuf) -> Result<Vec<String>> {
    let mut file_content = Vec::new();

    for line in read_to_string(filepath).unwrap().lines() {
        file_content.push(line.to_string())
    }

    Ok(file_content)
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
