mod social;
mod social_extractor;
mod social_models;
mod additional_functions;

use social::{
    SocialLink
};
use social_extractor::*;
use additional_functions::{
    return_links, pause, int_parse,
    file_build_release, extract_instagram_shortcode,
    social_type
};

use std::env;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::from_path(".env").or_else(|_| dotenvy::from_path("../.env"));

    let api_key: String = env::var("YOUTUBE_API").expect("Переменная YOUTUBE_API не задана");
    let session_id: String = env::var("INST_SESSION_ID").expect("Переменная INST_SESSION_ID не задана");

    let urls = return_links(&file_build_release("Ссылки.txt")).unwrap_or_default();

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    for url in &urls {
        match social_type(url) {
            Some(SocialLink::YouTube(yt)) => {
                if let Ok(response) = get_youtube_data(&client, extract_youtube_id(&yt.link).unwrap(), &api_key).await {
                    for item in response.items {
                        let views = item.statistics.view_count.as_deref().unwrap_or("0");
                        println!("{} : {}", int_parse(&views), yt.link);
                    }
                }
            }
            Some(SocialLink::TikTok(tt)) => {
                if let Ok(Some(data)) = get_tiktok_data(&client, &tt.link).await {
                    println!("{} : {}", int_parse(&data.play_count.to_string()), &tt.link);
                }
                else {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    if let Ok(Some(data)) = get_tiktok_data(&client, &tt.link).await {
                        println!("{} : {}", int_parse(&data.play_count.to_string()), &tt.link);
                    } else {
                        println!("0 : {}", &tt.link);
                    }
                }
                tokio::time::sleep(Duration::from_millis(400)).await;
            }
            Some(SocialLink::Instagram(inst)) => {
                let play_views = match extract_instagram_shortcode(&inst.link) {
                    Some(shortcode) => {
                        get_instagram_data(&client, &shortcode, &session_id)
                            .await
                            .unwrap_or(0)
                    }
                    None => 0,
                };

                println!("{} : {}", int_parse(&play_views.to_string()), &inst.link);
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
            None => {
                eprintln!("Неизвестный домен или неподдерживаемая ссылка: {}", url);
            }
        }
    }

    pause();
    Ok(())
}