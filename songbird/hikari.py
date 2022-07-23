from __future__ import annotations

from typing import Callable, Awaitable, Any

from hikari import snowflakes, VoiceEvent, GatewayBot
from hikari.api import VoiceComponent, VoiceConnection

from .songbird import Driver
from .voicebox_base import VoiceboxBase


class Voicebox(VoiceboxBase, VoiceConnection):
    """Hikari VoiceConnection using Songbird"""

    def __init__(
        self,
        driver: Driver,
        *,
        channel_id: snowflakes.Snowflake,
        guild_id: snowflakes.Snowflake,
        is_alive: bool,
        shard_id: int,
        owner: VoiceComponent,
        on_close: Callable[[Voicebox], Awaitable[None]],
    ) -> None:
        super().__init__(driver)

        self.__channel_id = channel_id
        self.__guild_id = guild_id
        self.__is_alive = is_alive
        self.__shard_id = shard_id
        self.__owner = owner
        self.__on_close = on_close


    @classmethod
    async def connect(cls, client: GatewayBot, guild_id: snowflakes.Snowflake, channel_id: snowflakes.Snowflake) -> Voicebox:
        return await client.voice.connect_to(
            guild_id,
            channel_id,
            voice_connection_type=Voicebox
        )

    @classmethod
    async def initialize(
        cls: type[Voicebox],
        channel_id: snowflakes.Snowflake,
        endpoint: str,
        guild_id: snowflakes.Snowflake,
        on_close: Callable[[Voicebox], Awaitable[None]],
        owner: VoiceComponent,
        session_id: str,
        shard_id: int,
        token: str,
        user_id: snowflakes.Snowflake,
        **kwargs: Any,
    ) -> Voicebox:
        driver = await Driver.create()
        await driver.connect(
            token=token,
            endpoint=endpoint,
            session_id=session_id,
            guild_id=guild_id,
            channel_id=channel_id,
            user_id=user_id
        )

        self = Voicebox(
            driver,
            channel_id=channel_id,
            guild_id=guild_id,
            is_alive=True,
            shard_id=shard_id,
            owner=owner,
            on_close=on_close,
        )

        return self

    @property
    def channel_id(self) -> snowflakes.Snowflake:
        """Return the ID of the voice channel this voice connection is in."""
        return self.__channel_id

    @property
    def guild_id(self) -> snowflakes.Snowflake:
        """Return the ID of the guild this voice connection is in."""
        return self.__guild_id

    @property
    def is_alive(self) -> bool:
        """Return `builtins.True` if the connection is alive."""
        return self.__is_alive

    @property
    def shard_id(self) -> int:
        """Return the ID of the shard that requested the connection."""
        return self.__shard_id

    @property
    def owner(self) -> VoiceComponent:
        """Return the component that is managing this connection."""
        return self.__owner

    async def disconnect(self) -> None:
        """Signal the process to shut down."""
        self.__is_alive = False
        await self.driver.leave()
        await self.__on_close(self)

    async def leave(self) -> None:
        """Does the same thing as disconnect"""
        await self.disconnect()

    async def join(self) -> None:
        """Wait for the process to halt before continuing."""

    async def notify(self, event: VoiceEvent) -> None:
        """Submit an event to the voice connection to be processed."""
