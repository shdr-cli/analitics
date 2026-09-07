import id_execute
import asyncio
import os

from dotenv import load_dotenv
from googleapiclient.discovery import build
from tqdm import tqdm
from TikTokApi import TikTokApi
from colorama import Fore, Back, Style, init

load_dotenv() # dotenv
init() # colorama

def returnLinks(path: str) -> list:
    """Возвращает массив ссылок из файла

    Args:
        path (str): Путь до файла

    Returns:
        list: Массив ссылок
    """
    with open(path, "r") as file:
        links = file.readlines()
    links = [link.strip() for link in links if link.strip()]
    return links

class YouTube:
    _YOUTUBE_API = os.getenv("YOUTUBE_API")

    def __init__(self) -> None:
        self.youtube = build("youtube", "v3", developerKey = self.__class__._YOUTUBE_API)

        links = returnLinks("youtube.txt")
        
        self.video_ids = [] # Максимум 50 за раз

        for video_url in links:
            self.video_ids.append(id_execute.extract_video_id(video_url))

        print(f"Всего Youtube ссылок: {Fore.YELLOW}{len(links)}{Style.RESET_ALL}")

    def printViews(self) -> None:
        pbar = tqdm(
            total=len(self.video_ids),
            desc="YouTube",
            position=0,
            leave=True,
            ncols=80
        )
        
        for video_id in self.video_ids:
            request = self.youtube.videos().list(
                part="snippet,statistics",
                id=video_id
            )
            response = request.execute()

            for video in response.get("items", []):
                title = video["snippet"]["title"]
                views = video["statistics"].get("viewCount", "Нет данных")
                
                tqdm.write(f"\n🎬 {title}")
                tqdm.write(f"📊 Просмотров: {int(views):,}")
                tqdm.write("-" * 30)
            
            pbar.update(1)
        
        pbar.close()

class TikTok:
    _MS_TOKEN_1 = os.getenv("MS_TOKEN_1")
    _MS_TOKEN_2 = os.getenv("MS_TOKEN_2")

    def __init__(self) -> None:
        self.ms_tokens = [self.__class__._MS_TOKEN_1, self.__class__._MS_TOKEN_2]

        links = returnLinks("tiktok.txt")
        self.video_urls = []

        for video_url in links:
            normal_url = video_url.split("?")[0]
            self.video_urls.append(normal_url)

        print(f"Всего TikTok ссылок: {Fore.YELLOW}{len(links)}{Style.RESET_ALL}")

    async def printViews(self) -> None:
        pbar = tqdm(
            total=len(self.video_urls),
            desc="TikTok",
            position=0,
            leave=True,
            ncols=80
        )

        async with TikTokApi() as tApi:
            await tApi.create_sessions(
                ms_tokens=self.ms_tokens,
                num_sessions=1,
                sleep_after=1,
                headless=False,
                browser="chromium"
            )

            # await asyncio.sleep(2)

            tqdm.write("Информация о тт видео:")

            for video_url in self.video_urls:
                await asyncio.sleep(1)
                video = tApi.video(url = video_url)

                video_data = await video.info()
                stats = video_data["stats"]
                author = video_data["author"]["nickname"]
                description = video_data.get("desc", "Нет описания")
                views = stats["playCount"]

                tqdm.write(f"\n🎬 Автор: {author}")
                tqdm.write(f"Описание: {description}")
                tqdm.write(f"👁️ Просмотров: {views:,}")
                tqdm.write("-" * 30)

                pbar.update(1)
            pbar.close()