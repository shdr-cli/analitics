import asyncio
import os

from platforms import YouTube, TikTok

async def MainProgram() -> None:
    YoutubeStat = YouTube()
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