from asyncio import gather
from pincer.client import Client

from pincer.core import GatewayDispatch, Gateway

from .songbird import Driver
from .voicebox_base import VoiceboxBase


class Voicebox(VoiceboxBase):
    def __init__(self, driver: Driver, shard: Gateway, guild_id: int) -> None:
        self.shard = shard
        self.guild_id = guild_id

        super().__init__(driver)

    @classmethod
    async def connect(cls, client: Client, shard: Gateway, guild_id: int, channel_id: int):
        """Creates a voicebox and joins a channel. This should always be used instead of :meth:`__init__`"""
        await shard.send(str(GatewayDispatch(
            4,
            {
                "guild_id": str(guild_id),
                "channel_id": str(channel_id),
                "self_mute": False,
                "self_deaf": False
            }
        )))

        state, server = await gather(
            client.wait_for(
                "on_voice_state_update", check=lambda state: state.user_id == client.bot.id and state.guild_id == guild_id  # type: ignore
            ),
            client.wait_for(
                "on_voice_server_update", check=lambda server: server.guild_id == guild_id
            )
        )

        driver = await Driver.create()

        await driver.connect(
            token=server.token,
            endpoint=server.endpoint,
            session_id=state.session_id,
            guild_id=server.guild_id,
            channel_id=state.channel_id,
            user_id=state.user_id
        )

        return cls(driver, shard, guild_id)

    async def leave(self):
        """Leaves the voice channel that the driver is in"""
        await self.shard.send(str(GatewayDispatch(
            4,
            {
                "guild_id": self.guild_id,
                "channel_id": None,
                "self_mute": False,
                "self_deaf": False
            }
        )))
