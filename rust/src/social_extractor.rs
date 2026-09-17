use crate::social_models::*;
use url::Url;

use crate::additional_functions:: {
    shortcode_to_media_id
};

pub async fn get_instagram_data(
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

pub async fn get_youtube_data(
    client: &reqwest::Client,
    video_id: String,
    api_key: &str,
) -> Result<YouTubeResponse, reqwest::Error> {
    let url = format!(
        "https://www.googleapis.com/youtube/v3/videos?part=statistics&id={}&key={}",
        video_id, api_key
    );

    client.get(url).send().await?.json::<YouTubeResponse>().await
}

pub async fn get_tiktok_data(
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

pub fn extract_youtube_id(url: &str) -> Option<String> {
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