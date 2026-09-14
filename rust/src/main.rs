mod social_models;
use social_models::*;

use regex::Regex;
use std::io;
use std::env;
use std::fs;
use std::sync::LazyLock;
use std::time::Duration;
use url::Url;
use num_format::{Locale, ToFormattedString};

fn pause() {
    println!("Нажмите Enter, чтобы выйти...");
    
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
}

fn int_parse(text: &str) -> String {
    let text_int: u64 = text.parse().unwrap();
    return text_int.to_formatted_string(&Locale::en);
}

fn file_build_release(path: &str) -> String {
    let build_path: String = format!("../{}", path);

    if return_links(path).is_ok() {
        return String::from(path);
    } else {
        return build_path;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match dotenvy::from_path(".env") {
        Ok(_) => {}
        Err(_) => {
            match dotenvy::from_path("../.env") {
                Ok(_) => { }
                Err(_) => {
                    eprintln!("Файл .env не найден");
                }
            }
        }
    }

    let api_key: String = env::var("YOUTUBE_API").expect("Переменная YOUTUBE_API не задана");
    let session_id: String = env::var("INST_SESSION_ID").expect("Переменная INST_SESSION_ID не задана");

    let urls_yt = return_links(&file_build_release("youtube.txt")).unwrap_or_default();
    let urls_tt = return_links(&file_build_release("tiktok.txt")).unwrap_or_default();
    let urls_inst = return_links(&file_build_release("instagram.txt")).unwrap_or_default();

    // let urls = return_links(&file_build_release("Ссылки.txt")).unwrap_or_default();

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        // .redirect(reqwest::redirect::Policy::none())
        .build()?;

    // YouTube
    println!("YOUTUBE");
    let yt_ids: Vec<String> = urls_yt
        .iter()
        .filter_map(|url| extract_youtube_id(url))
        .collect();

    for chunk in yt_ids.chunks(50) {
        if let Ok(response) = fetch_youtube_data_batch(&client, chunk, &api_key).await {
            for item in response.items {
                let views = item.statistics.view_count.as_deref().unwrap_or("0");
                println!("{}: {}", int_parse(&views), item.id);
            }
        }
    }

    // TikTok
    println!("\nTIKTOK");
    for url in &urls_tt {
        if let Ok(Some(data)) = fetch_tiktok_data(&client, url).await {
            println!("{}: {}", int_parse(&data.play_count.to_string()), url);
        } else {
            tokio::time::sleep(Duration::from_secs(1)).await;
            if let Ok(Some(data)) = fetch_tiktok_data(&client, url).await {
                println!("{}: {}", int_parse(&data.play_count.to_string()), url);
            } else {
                println!("0: {}", url);
            }
        }
        tokio::time::sleep(Duration::from_millis(400)).await;
    }

    // Instagram
    println!("\nINSTAGRAM");
    for url in &urls_inst {
        let play_views = match extract_instagram_shortcode(url) {
            Some(shortcode) => {
                fetch_instagram_views(&client, &shortcode, &session_id)
                    .await
                    .unwrap_or(0)
            }
            None => 0,
        };

        println!("{} : {}", int_parse(&play_views.to_string()), url);
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    pause();
    Ok(())
}

fn shortcode_to_media_id(shortcode: &str) -> Option<u64> {
    const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut id: u64 = 0;

    for ch in shortcode.chars() {
        let value = ALPHABET.find(ch)? as u64;
        id = id.checked_mul(64)?.checked_add(value)?;
    }

    Some(id)
}

fn extract_instagram_shortcode(input_url: &str) -> Option<String> {
    static RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"/(?:reel|reels|p|tv)/([A-Za-z0-9_-]+)").unwrap()
    });

    RE.captures(input_url)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
}

async fn fetch_instagram_views(
    client: &reqwest::Client,
    shortcode: &str,
    session_id: &str,
) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    let media_id = match shortcode_to_media_id(shortcode) {
        Some(id) => id,
        None => return Ok(0),
    };

    let url = format!("https://i.instagram.com/api/v1/media/{}/info/", media_id);

    let cookie_str = if session_id.contains('=') {
        session_id.trim().to_string()
    } else {
        format!("sessionid={};", session_id.trim())
    };

    let res = client
        .get(&url)
        .header(
            "User-Agent",
            "Instagram 320.0.0.35.109 Android (33/13; 420dpi; 1080x2400; samsung; SM-G991N)",
        )
        .header("Cookie", cookie_str)
        .header("X-IG-App-ID", "1217981644879628")
        .header("Accept", "*/*")
        .send()
        .await?;

    if !res.status().is_success() {
        return Ok(0);
    }

    let inst_data: IgApiResponse = res.json().await?;

    if let Some(item) = inst_data.items.and_then(|items| items.into_iter().next()) {
        let play_views = item
            .play_count
            .or(item.view_count)
            .or(item.video_play_count)
            .or(item.fb_play_count)
            .unwrap_or(0);

        return Ok(play_views);
    }

    return Ok(0);
}

async fn fetch_youtube_data_batch(
    client: &reqwest::Client,
    video_ids: &[String],
    api_key: &str,
) -> Result<YouTubeResponse, reqwest::Error> {
    let ids_joined = video_ids.join(",");
    let url = format!(
        "https://www.googleapis.com/youtube/v3/videos?part=statistics&id={}&key={}",
        ids_joined, api_key
    );

    client.get(url).send().await?.json::<YouTubeResponse>().await
}

async fn fetch_tiktok_data(
    client: &reqwest::Client,
    video_url: &str,
) -> Result<Option<TikWmData>, reqwest::Error> {
    let api_url = format!("https://www.tikwm.com/api/?url={}", video_url);

    let response = client
        .get(&api_url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await?
        .json::<TikWmResponse>()
        .await?;

    if response.code == 0 {
        Ok(response.data)
    } else {
        Ok(None)
    }
}

fn extract_youtube_id(url: &str) -> Option<String> {
    let parsed: Url = Url::parse(url).ok()?;
    let host: &str = parsed.host_str()?;
    let path: &str = parsed.path();

    if !["www.youtube.com", "youtube.com", "youtu.be"].contains(&host) {
        return None;
    }

    if host == "youtu.be" {
        return Some(path.trim_start_matches('/').to_string());
    }

    if path.starts_with("/watch") {
        for (key, value) in parsed.query_pairs() {
            if key == "v" {
                return Some(value.to_string());
            }
        }
    }

    for prefix in ["/shorts/", "/embed/", "/v/"] {
        if path.contains(prefix) {
            if let Some(id) = path.split(prefix).nth(1) {
                return Some(id.to_string());
            }
        }
    }

    None
}

fn return_links(path: &str) -> Result<Vec<String>, std::io::Error> {
    let content = fs::read_to_string(path)?;
    Ok(content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(String::from)
        .collect())
}