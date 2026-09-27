use crate::social::SocialLink;

use std::fs;
use std::io;
use num_format::{Locale, ToFormattedString};

pub fn pause() {
    println!("\nНажмите Enter, чтобы выйти...");
    
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
}

pub fn social_type(url: &str) -> Option<SocialLink>{
    if url.contains("youtube.com") || url.contains("youtu.be") {
        return Some(SocialLink::YouTube);
    } else if url.contains("tiktok.com") {
        return Some(SocialLink::TikTok);
    } else {
        return None;
    }
}

pub fn int_parse(text: &str) -> String {
    let text_int: u64 = text.parse().unwrap();
    return text_int.to_formatted_string(&Locale::en);
}

pub fn file_build_release(path: &str) -> String {
    let build_path: String = format!("../{}", path);

    if return_links(path).is_ok() {
        return String::from(path);
    } else {
        return build_path;
    }
}

pub fn return_links(path: &str) -> Result<Vec<String>, std::io::Error> {
    let content = fs::read_to_string(path)?;
    Ok(content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(String::from)
        .collect())
}