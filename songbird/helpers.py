"""I use Arch btw so this module exists to remove bloat"""

from typing import overload
from .songbird import Source


@overload
async def ffmpeg(filepath: str, pre_input_args=None, args=None) -> Source: ...


async def ffmpeg(filepath: str, **kwargs) -> Source:
    """Builds a ffmpeg source"""
    return await Source.ffmpeg(filepath, **kwargs)


async def ytdl(url: str) -> Source:
    """Builds a ytdl source"""
    return await Source.ytdl(url)
