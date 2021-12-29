"""I use Arch btw so this module exists to remove bloat"""

from .songbird import Source

async def ffmpeg(filepath: str) -> Source:
    """Builds a ffmpeg source"""
    return await Source.ffmpeg(filepath)

async def ytdl(url: str) -> Source:
    """Builds a ytdl source"""
    return await Source.ytdl(url)
