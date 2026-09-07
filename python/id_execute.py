from urllib.parse import urlparse, parse_qs

def extract_video_id(url: str) -> str | None:
    parsed = urlparse(url)

    if parsed.hostname in ("www.youtube.com", "youtube.com", "youtu.be"):
        if parsed.hostname == "youtu.be":
            return parsed.path.lstrip("/")
        
        if parsed.path.startswith("/watch"):
            params = parse_qs(parsed.query)
            video_id = params.get("v", [None])[0]
            if video_id:
                return video_id
        
        if "/embed/" in parsed.path:
            return parsed.path.split("/embed/")[-1].split("/")[0]
        
        if "/shorts/" in parsed.path:
            video_id = parsed.path.split("/shorts/")[-1].split("/")[0]
            video_id = video_id.split("?")[0]
            return video_id
        
        if parsed.path.startswith("/v/"):
            return parsed.path.split("/v/")[-1].split("/")[0]
        
    return None