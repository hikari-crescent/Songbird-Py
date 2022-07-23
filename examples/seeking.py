from asyncio import sleep

import hikari
from songbird import Bitrate, CompressedSource, RestartableSource, ytdl
from songbird.hikari import Voicebox


bot = hikari.GatewayBot("...")


@bot.listen()
async def restartable_example(_: hikari.ShardReadyEvent) -> None:
    voice = await Voicebox.connect(bot, YOUR_GUILD_ID, YOUR_CHANNEL_ID)

    restartable = await RestartableSource.ytdl("https://www.youtube.com/watch?v=r25MAkzkTF4", False)
    track_handle = await voice.play_source(restartable.into_source())

    print(track_handle.is_seekable)  # True

    await sleep(1)

    # Seek to 40s into the track
    track_handle.seek_time(40)
    await sleep(5)

    # Seek back to the start
    track_handle.seek_time(0)


async def compressed_example() -> None:
    voice = await Voicebox.connect(bot, YOUR_GUILD_ID, YOUR_CHANNEL_ID)

    restartable = await CompressedSource.from_source(await ytdl("https://www.youtube.com/watch?v=r25MAkzkTF4"), Bitrate.AUTO)
    track_handle = await voice.play_source(restartable.into_source())

    print(track_handle.is_seekable)  # True

    await sleep(1)

    # Seek to 40s into the track
    track_handle.seek_time(40)
    await sleep(5)

    # Seek back to the start
    track_handle.seek_time(0)