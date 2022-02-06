"""Static help functions."""

# FIXME: Typing

import typing

from .songbird import Source


@typing.overload
async def ffmpeg(filepath: str, pre_input_args=None, args=None) -> Source: ...


async def ffmpeg(filepath: str, **kwargs: typing.Any) -> Source:
    """Builds a ffmpeg source"""
    return await Source.ffmpeg(filepath, **kwargs)


async def ytdl(url: str) -> Source:
    """Builds a ytdl source"""
    return await Source.ytdl(url)
