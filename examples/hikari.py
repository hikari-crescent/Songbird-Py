from asyncio import sleep

import hikari
from songbird import ytdl
from songbird.hikari import Voicebox


bot = hikari.GatewayBot("...")


@bot.listen()
async def ping(event: hikari.ShardReadyEvent) -> None:
    voice = await Voicebox.connect(bot, YOUR_GUILD_ID, YOUR_CHANNEL_ID)

    track_handle = await voice.play_source(await ytdl("https://www.youtube.com/watch?v=r25MAkzkTF4"))

    await sleep(5)
    # Doesn't need to be awaited!
    track_handle.pause()
    await sleep(5)
    track_handle.play()


bot.run()
