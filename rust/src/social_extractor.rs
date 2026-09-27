use crate::social_models::*;
use url::Url;

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