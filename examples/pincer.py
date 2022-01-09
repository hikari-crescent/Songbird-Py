from typing import Any, Dict

from pincer import Client, Intents
from pincer.objects import MessageContext
from pincer.commands import command, CommandArg, Description

from songbird import ytdl, Driver, PlayMode, TrackHandle, Queue
from songbird.pincer import Voicebox

token = "..."


class Bot(Client):
    def __init__(self, token: str, intents: Intents = None):
        self.queues: Dict[int, Queue] = {}
        super().__init__(token, intents=intents)

    @Client.event
    async def on_ready(self, shard):

        guild_id = 750862883075915826
        channel_id = 919040848237330462

        voice = await Voicebox.connect(self, shard, guild_id, channel_id)

        self.queues[guild_id] = Queue(voice, on_fail=self.on_fail_to_play)

    async def on_fail_to_play(self, driver: Driver, video: Any):
        # You can send a message to a user here or something
        print("Failed to play video")

    @Client.event
    async def on_message(self, msg):
        print(msg)

    @command
    async def play(
        self,
        ctx: MessageContext,
        url: CommandArg[str, Description("Play a youtube video")]
    ):
        self.queues[ctx.guild_id] += [ytdl(url)]
        return "Added song to queue"

    @command
    async def skip(self, ctx: MessageContext):
        self.queues[ctx.guild_id].skip()
        return "Skipped to the next song!"

    @command
    async def toggle_playing(self, ctx: MessageContext):
        track_handle: TrackHandle = self.queues[ctx.guild_id].track_handle

        info = await track_handle.get_info()

        if info.playing == PlayMode.Play:
            track_handle.pause()
            return "Paused the current song!"

        track_handle.play()
        return "Unpaused the current song!"


Bot(token, intents=Intents.all()).run()


class Bot(Client):
    def __init__(self, token: str, intents: Intents = None):
        self.queues: Dict[int, Queue] = {}
        super().__init__(token, intents=intents)

    @Client.event
    async def on_ready(self, shard):

        guild_id = YOUR_GUILD_ID
        channel_id = YOUR_CHANNEL_ID

        voice = await Voicebox.connect(self, shard, guild_id, channel_id)

        self.queues[guild_id] = Queue(voice, on_fail=self.on_fail_to_play)

    async def on_fail_to_play(self, driver: Driver, video: Any):
        # You can send a message to a user here or something
        print("Failed to play video")

    @Client.event
    async def on_message(self, msg):
        print(msg)

    @command
    async def play(
        self,
        ctx: MessageContext,
        url: CommandArg[str, Description("Play a youtube video")]
    ):
        self.queues[ctx.guild_id] += [ytdl(url)]
        return "Added song to queue"

    @command
    async def skip(self, ctx: MessageContext):
        self.queues[ctx.guild_id].skip()
        return "Skipped to the next song!"

    @command
    async def toggle_playing(self, ctx: MessageContext):
        track_handle: TrackHandle = self.queues[ctx.guild_id].track_handle

        info = await track_handle.get_info()

        if info.playing == PlayMode.Play:
            track_handle.pause()
            return "Paused the current song!"

        track_handle.play()
        return "Unpaused the current song!"


Bot(token, intents=Intents.all()).run()
