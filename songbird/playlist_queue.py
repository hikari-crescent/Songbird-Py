from asyncio import create_subprocess_shell
from asyncio.subprocess import PIPE
import json
from typing import List

from songbird.helpers import ytdl


async def get_playlist(playlist: str) -> List[str]:
    cmd = f"yt-dlp '{playlist}' --flat-playlist --dump-single-json"

    proc = await create_subprocess_shell(cmd, stdout=PIPE, stderr=PIPE)
    stdout, _ = await proc.communicate()

    out = []
    if stdout:
        out.extend(
            WillBeYtdl(entry["url"])
            for entry in json.loads(stdout)["entries"]
        )

    return out


class WillBeYtdl:
    def __init__(self, url: str) -> None:
        self.url = url

    def __await__(self):
        return ytdl(self.url).__await__()
