from urllib.parse import urlparse, parse_qs

def extract_video_id(url: str) -> str | None:
    """
    Извлекает ID видео из ссылок YouTube и TikTok.
    
    YouTube:
    - https://www.youtube.com/watch?v=VIDEO_ID
    - https://youtu.be/VIDEO_ID
    - https://www.youtube.com/embed/VIDEO_ID
    
    TikTok:
    - https://www.tiktok.com/@username/video/VIDEO_ID
    - https://www.tiktok.com/@username/video/VIDEO_ID?is_from_webapp=1
    - https://vm.tiktok.com/ABCDEFG/ (короткая ссылка)
    - https://vt.tiktok.com/ABCDEFG/ (короткая ссылка)
    """
    
    parsed = urlparse(url)

    if parsed.hostname in ("www.youtube.com", "youtube.com"):
        return parse_qs(parsed.query).get("v")[0]

    if "tiktok.com" in parsed.hostname:
        if "/video/" in parsed.path:
            video_id = parsed.path.split("/video/")[-1].split("/")[0]
            return video_id
    return None