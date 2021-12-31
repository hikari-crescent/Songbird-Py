from typing import List
from pincer import Client
from pincer.objects.app.intents import Intents
from songbird import ytdl
from songbird.pincer import Voicebox

token = "..."


class Bot(Client):
    def __init__(self, token: str, intents: Intents = None):
        self.voiceboxes: List[int, Voicebox] = {}
        super().__init__(token, intents=intents)

    @Client.event
    async def on_ready(self, shard):

        guild_id = YOUR_GUILD_ID
        channel_id = YOUR_CHANNEL_ID

        voice = await Voicebox.connect(self, shard, guild_id, channel_id)

        self.voiceboxes[hash((guild_id, channel_id))] = voice

        await voice.play_source(await ytdl("https://www.youtube.com/watch?v=3Rl-Ty5jv8o"))

    @Client.event
    async def on_message(self, msg):
        print(msg)


Bot(token, intents=Intents.all()).run()
