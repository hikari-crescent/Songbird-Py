from __future__ import annotations

from asyncio import create_subprocess_shell
from asyncio.subprocess import PIPE
import json
from typing import Any, Dict, List

from songbird.helpers import ytdl
from songbird.songbird import Metadata


async def get_playlist(playlist: str) -> List[YoutubeVideo]:
    cmd = f"yt-dlp '{playlist}' --flat-playlist --dump-single-json"

    proc = await create_subprocess_shell(cmd, stdout=PIPE, stderr=PIPE)
    stdout, _ = await proc.communicate()

    out: List[YoutubeVideo] = []
    if stdout:
        out.extend(
            YoutubeVideo(entry) for entry in json.loads(stdout)["entries"]
        )

    return out


class YoutubeVideo:
    def __init__(self, video: Dict[str, Any]) -> None:
        self.metadata = Metadata(
            channel=video.get("channel"),
            duration=video.get("duration"),
            source_url=video.get("url"),
            title=video.get("title"),
            thumbnail=video.get("thumbnails", [{"url": None}])[0]["url"]
        )

    def __await__(self):
        return ytdl(self.metadata.source_url).__await__()
