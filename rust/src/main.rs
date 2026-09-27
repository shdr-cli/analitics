mod social;
mod social_extractor;
mod social_models;
mod additional_functions;

use social::{
    SocialLink
};
use social_extractor::*;
use additional_functions::*;
use std::env;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let _ = dotenvy::from_path(".env").or_else(|_| dotenvy::from_path("../.env"));

    let api_key: String = env::var("YOUTUBE_API").expect("Переменная YOUTUBE_API не задана");

    let urls = return_links(&file_build_release("Ссылки.txt")).unwrap_or_default();

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build().unwrap();

    for url in &urls {
        match social_type(url) {
            Some(SocialLink::YouTube) => {
                if let Ok(response) = get_youtube_data(&client, extract_youtube_id(&url).unwrap(), &api_key).await {
                    for item in response.items {
                        let views = item.statistics.view_count.as_deref().unwrap_or("0");
                        println!("{} : {}", int_parse(&views), url);
                    }
                }
            }
            Some(SocialLink::TikTok) => {
                if let Ok(Some(data)) = get_tiktok_data(&client, &url).await {
                    println!("{} : {}", int_parse(&data.play_count.to_string()), url);
                }
                else {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    if let Ok(Some(data)) = get_tiktok_data(&client, &url).await {
                        println!("{} : {}", int_parse(&data.play_count.to_string()), url);
                    } else {
                        println!("0 : {}", url);
                    }
                }
                tokio::time::sleep(Duration::from_millis(400)).await;
            }
            None => {
                eprintln!("Неизвестный домен или неподдерживаемая ссылка: {}", url);
            }
        }
    }

    pause();
}