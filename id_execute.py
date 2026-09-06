import re
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
    
    # === YouTube ===
    youtube_patterns = [
        r"youtube\.com/watch\?v=([^&]+)",      # стандартная
        r"youtu\.be/([^?]+)",                  # короткая
        r"youtube\.com/embed/([^?]+)",         # встраиваемая
        r"youtube\.com/v/([^?]+)"              # старый формат
    ]
    
    for pattern in youtube_patterns:
        match = re.search(pattern, url)
        if match:
            return match.group(1)
    
    # YouTube через parse_qs
    parsed = urlparse(url)
    if parsed.hostname in ("www.youtube.com", "youtube.com"):
        video_id = parse_qs(parsed.query).get("v", [None])[0]
        if video_id:
            return video_id
    
    # === TikTok ===
    tiktok_patterns = [
        # Полная ссылка: /@username/video/ID или /@username/photo/ID
        r"tiktok\.com/@[^/]+/(?:video|photo)/(\d+)",
        
        # Короткие ссылки: vm.tiktok.com/ID или vt.tiktok.com/ID
        r"(?:vm|vt)\.tiktok\.com/([A-Za-z0-9]+)",
        
        # Альтернативный формат: /@username/video/ID?params
        r"tiktok\.com/@[^/]+/video/(\d+)",
    ]
    
    for pattern in tiktok_patterns:
        match = re.search(pattern, url)
        if match:
            return match.group(1)
    
    # TikTok через parse_qs (если ID передан как параметр)
    if parsed.hostname in ("www.tiktok.com", "tiktok.com", "vm.tiktok.com", "vt.tiktok.com"):
        # Иногда ID может быть в параметрах
        video_id = parse_qs(parsed.query).get("video_id", [None])[0]
        if video_id:
            return video_id
    
    return None