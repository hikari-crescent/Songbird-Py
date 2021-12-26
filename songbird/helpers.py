"""I use Arch btw so this module exists to remove bloat"""

from .songbird import Source

def ffmpeg(filepath: str) -> Source:
    """Builds a ffmpeg source"""
    return Source.ffmpeg(filepath)

def ytdl(url: str) -> Source:
    """Builds a ytdl source"""
    return Source.ytdl(url)
