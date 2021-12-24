import hikari
from songbird import Source
from songbird.hikari import Voicebox

from asyncio import sleep

bot = hikari.GatewayBot("...")


@bot.listen()
async def ping(event: hikari.ShardReadyEvent) -> None:
    voice = await bot.voice.connect_to(
        # Both of the these can be `int` or `Snowflake`
        YOUR_GUILD_ID,
        YOUR_CHANNEL_ID,
        voice_connection_type=Voicebox
    )

    handle = await voice.play_source(Source.ytdl("https://www.youtube.com/watch?v=r25MAkzkTF4"))

    sleep(5)

    # Pausing is synchronous and non-blocking.
    handle.pause()

    sleep(5)

    handle.play()

    sleep(5)

    # Voicebox is garbage collected here so It stops playing


bot.run()
