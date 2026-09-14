pub struct YouTubeLink {
    pub link: String,
}
pub struct TikTokLink {
    pub link: String,
}
pub struct InstagramLink {
    pub link: String,
}
pub enum SocialLink {
    YouTube(YouTubeLink),
    TikTok(TikTokLink),
    Instagram(InstagramLink),
}