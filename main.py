import id_execute
import asyncio
import os
import pprint
from dotenv import load_dotenv
from googleapiclient.discovery import build
from TikTokApi import TikTokApi
from colorama import Fore, Back, Style, init

load_dotenv() # dotenv
init() # colorama

class Youtube:
    _YOUTUBE_API = os.getenv("YOUTUBE_API")

    def __init__(self) -> None:
        self.youtube = build("youtube", "v3", developerKey = self.__class__._YOUTUBE_API)

        with open("youtube.txt", "r") as file:
            links = file.readlines()

        links = [link.strip() for link in links if link.strip()]
        self.video_ids = [] # Максимум 50 за раз

        for video_url in links:
            self.video_ids.append(id_execute.extract_video_id(video_url))

        print(f"Всего Youtube ссылок: {Fore.YELLOW}{len(links)}{Style.RESET_ALL}")

    def printViews(self) -> None:
        self.request = self.youtube.videos().list(
            part="snippet,statistics",
            id=",".join(self.video_ids)
        )
        self.response = self.request.execute()

        for video in self.response.get("items", []):
            self.title = video["snippet"]["title"]
            self.views = video["statistics"].get("viewCount", "Нет данных")
            
            print("\n🎬 {title}".format(title = self.title))
            print("📊 Просмотров: {views:,}".format(views = int(self.views)))
            print("-" * 30)

class TikTok:
    _MS_TOKEN_1 = os.getenv("MS_TOKEN_1")
    _MS_TOKEN_2 = os.getenv("MS_TOKEN_2")

    def __init__(self) -> None:
        self.ms_tokens = [self.__class__._MS_TOKEN_1, self.__class__._MS_TOKEN_2]

        with open("tiktok.txt", "r") as file:
            links = file.readlines()

        links = [link.strip() for link in links if link.strip()]
        self.video_urls = []

        for video_url in links:
            # self.video_ids.append(id_execute.extract_video_id(video_url))
            normal_url = video_url.split("?")[0]
            self.video_urls.append(normal_url)

        print(f"Всего TikTok ссылок: {Fore.YELLOW}{len(links)}{Style.RESET_ALL}")

    async def printViews(self) -> None:
        async with TikTokApi() as tApi:
            await tApi.create_sessions(
                ms_tokens=self.ms_tokens,
                num_sessions=1,
                sleep_after=1,
                headless=False,
                browser="chromium"
            )

            # await asyncio.sleep(2)

            for video_url in self.video_urls:
                await asyncio.sleep(1)
                video = tApi.video(url = video_url)

                video_data = await video.info()
                stats = video_data["stats"]
                author = video_data["author"]["nickname"]
                description = video_data.get("desc", "Нет описания")
                views = stats["playCount"]

                print(f"\n🎬 Автор: {author}")
                print(f"Описание: {description}")
                print(f"👁️ Просмотров: {views:,}")
                print("-" * 30)

async def MainProgram() -> None:
    YoutubeStat = Youtube()
    TikTokStat = TikTok()

    YoutubeStat.printViews()
    await TikTokStat.printViews()

def checkGetenv() -> bool:
    toInit = True
    if os.getenv("YOUTUBE_API") is None:
        print("Переменная YOUTUBE_API отсутствует!")
        toInit = False
    if os.getenv("MS_TOKEN_1") is None:
        print("Переменная MS_TOKEN_1 отсутствует!")
        toInit = False
    if os.getenv("MS_TOKEN_2") is None:
        print("Переменная MS_TOKEN_2 отсутствует!")
        toInit = False
    return toInit

if __name__ == "__main__":
    if checkGetenv():
        asyncio.run(MainProgram())