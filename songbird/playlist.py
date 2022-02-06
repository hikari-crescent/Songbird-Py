from __future__ import annotations

import json
from asyncio import create_subprocess_shell
from asyncio.subprocess import PIPE
from typing import (
    Any,
    Generator,
)

from songbird.helpers import ytdl
from songbird.songbird import Metadata


async def get_playlist(playlist: str) -> list[YoutubeVideo]:
    cmd = f"yt-dlp '{playlist}' --flat-playlist --dump-single-json"

    proc = await create_subprocess_shell(cmd, stdout=PIPE, stderr=PIPE)
    stdout, stderr = await proc.communicate()

    if stderr:
        raise ValueError(stderr)

    out: list[YoutubeVideo] = []
    if stdout:
        out.extend(
            YoutubeVideo(entry) for entry in json.loads(stdout)["entries"]
        )  # TODO: Errors with youtube-dl outs

    return out


class YoutubeVideo:
    def __init__(self, video: dict[str, Any]) -> None:
        self.metadata = Metadata(
            channel=video.get("channel"),
            duration=video.get("duration"),
            source_url=video.get("url"),
            title=video.get("title"),
            thumbnail=video.get("thumbnails", [{"url": None}])[0]["url"]
        )

    def __await__(self) -> Generator[Any, None, Any] | None:
        if self.metadata.source_url is not None:
            return ytdl(self.metadata.source_url).__await__()
        return None
