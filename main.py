import id_execute
from googleapiclient.discovery import build

YOUTUBE_API = "AIzaSyCuBNop40nkdWURRk1DEoKmM4lGT_NRSnE"

youtube = build("youtube", "v3", developerKey=YOUTUBE_API)

with open("youtube.txt", "r") as file:
    links = file.readlines()

links = [link.strip() for link  in links if link.strip()]
video_ids = [] # Максимум 50 за раз

for video_url in links:
    video_ids.append(id_execute.extract_video_id(video_url))

request = youtube.videos().list(
    part="snippet,statistics",
    id=",".join(video_ids)
)
response = request.execute()

for video in response.get("items", []):
    title = video["snippet"]["title"]
    views = video["statistics"].get("viewCount", "Нет данных")
    
    print("\n🎬 {title}".format(title=title))
    print("📊 Просмотров: {views}".format(views=views))
    print("-" * 20)