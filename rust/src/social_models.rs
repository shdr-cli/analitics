use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct VideoStatistics {
    #[serde(rename = "viewCount")]
    pub view_count: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct VideoItem {
    pub statistics: VideoStatistics,
}

#[derive(Deserialize, Debug)]
pub struct YouTubeResponse {
    pub items: Vec<VideoItem>,
}

#[derive(Deserialize, Debug)]
pub struct TikWmData {
    pub play_count: u64,
}

#[derive(Deserialize, Debug)]
pub struct TikWmResponse {
    pub code: i32,
    pub data: Option<TikWmData>,
}

#[derive(Deserialize, Debug)]
pub struct IgMediaItem {
    pub play_count: Option<u64>,
    pub view_count: Option<u64>,
    pub video_play_count: Option<u64>,
    pub fb_play_count: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct IgApiResponse {
    pub items: Option<Vec<IgMediaItem>>,
}