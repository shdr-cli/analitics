import re
from urllib.parse import urlparse, parse_qs

def extract_video_id(url) -> str:
    """Извлекает ID видео из разных форматов ссылок"""
    # Обрабатываем разные форматы
    patterns = [
        r'youtube\.com/watch\?v=([^&]+)',  # стандартная ссылка
        r'youtu\.be/([^?]+)',              # короткая ссылка
        r'youtube\.com/embed/([^?]+)',     # встраиваемая
        r'youtube\.com/v/([^?]+)'          # старый формат
    ]
    
    for pattern in patterns:
        match = re.search(pattern, url)
        if match:
            return match.group(1)
    
    # Если не сработало, пробуем через parse_qs
    parsed = urlparse(url)
    if parsed.hostname in ('www.youtube.com', 'youtube.com'):
        return parse_qs(parsed.query).get('v', [None])[0]
    
    return None