"""I use Arch btw so this module exists to remove bloat"""

from .songbird import Source


async def ffmpeg(filepath: str, pre_input_args=None, args=None) -> Source:
    """Builds a ffmpeg source"""
    kwargs = {}
    if args:
        kwargs["args"] = args
    if pre_input_args:
        kwargs["pre_input_args"] = pre_input_args
    return await Source.ffmpeg(filepath, **kwargs)


async def ytdl(url: str) -> Source:
    """Builds a ytdl source"""
    return await Source.ytdl(url)
