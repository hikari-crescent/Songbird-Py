from typing import Dict
from hikari.snowflakes import Snowflake
from lightbulb import command, option, implements, SlashCommand, BotApp, Context

from songbird import ytdl, Queue
from songbird.hikari import Voicebox

bot = BotApp(token="...")

queues: Dict[Snowflake, Queue] = {}


@bot.command
@command(description="Make the bot join your voice channel", name="join")
@implements(SlashCommand)
async def join(ctx: Context) -> None:
    voice = await bot.voice.connect_to(
        ctx.guild_id,
        bot.cache.get_voice_state(ctx.guild_id, ctx.author).channel_id,
        voice_connection_type=Voicebox
    )

    q = Queue(voice)
    queues[ctx.guild_id] = q

    await ctx.respond("Joined the channel!")


@bot.command
@option("song", "youtube url", str)
@command(description="Play a song", name="play")
@implements(SlashCommand)
async def play(ctx: Context):
    queues[ctx.guild_id].append(ytdl(ctx.options.song))
