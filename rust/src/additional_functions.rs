use crate::social::{
    YouTubeLink, TikTokLink, InstagramLink,
    SocialLink
};

use std::fs;
use std::io;
use num_format::{Locale, ToFormattedString};
use regex::Regex;
use std::sync::LazyLock;

pub fn pause() {
    println!("\nНажмите Enter, чтобы выйти...");
    
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
}

pub fn social_type(url: &str) -> Option<SocialLink>{
    if url.contains("youtube.com") || url.contains("youtu.be") {
        Some(SocialLink::YouTube(YouTubeLink {
            link: url.to_string(),
        }))
    } else if url.contains("tiktok.com") {
        Some(SocialLink::TikTok(TikTokLink {
            link: url.to_string(),
        }))
    } else if url.contains("instagram.com") {
        Some(SocialLink::Instagram(InstagramLink {
            link: url.to_string(),
        }))
    } else {
        None
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

pub fn extract_instagram_shortcode(input_url: &str) -> Option<String> {
    static RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"/(?:reel|reels|p|tv)/([A-Za-z0-9_-]+)").unwrap()
    });

    RE.captures(input_url)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
}

pub fn shortcode_to_media_id(shortcode: &str) -> Option<u64> {
    const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut id: u64 = 0;

    for ch in shortcode.chars() {
        let value = ALPHABET.find(ch)? as u64;
        id = id.checked_mul(64)?.checked_add(value)?;
    }

    Some(id)
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