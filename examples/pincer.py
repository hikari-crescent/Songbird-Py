from typing import Dict

from pincer import Client
from pincer.objects.app.intents import Intents
from songbird import ytdl
from songbird.pincer import Voicebox

token = "..."


class Bot(Client):

    def __init__(self, token: str, intents: Intents = None):
        self.voiceboxes: Dict[int, Voicebox] = {}
        super().__init__(token, intents=intents)

    @Client.event
    async def on_ready(self, shard):
        voice = await Voicebox.connect(self, shard, YOUR_GUILD_ID, YOUR_CHANNEL_ID)

        self.voiceboxes[YOUR_GUILD_ID] = voice

        await voice.play_source(await ytdl("https://www.youtube.com/watch?v=3Rl-Ty5jv8o"))


Bot(token, intents=Intents.all()).run()
