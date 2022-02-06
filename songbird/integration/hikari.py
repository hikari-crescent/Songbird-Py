from __future__ import annotations

from typing import (
    Callable,
    Awaitable,
    Any,
    TypeVar,
    Type,
)

from hikari import snowflakes, VoiceEvent, GatewayBot
from hikari.api import VoiceComponent, VoiceConnection

from ..songbird import Driver
from songbird.integration.voicebox_base import VoiceboxBase


VoiceBoxType = TypeVar("VoiceBoxType", bound="HikariVoicebox")


class HikariVoicebox(VoiceboxBase, VoiceConnection):
    """Hikari VoiceConnection using Songbird."""

    _channel_id: snowflakes.Snowflake
    _guild_id: snowflakes.Snowflake
    _is_alive: bool
    _shard_id: int
    _owner: VoiceComponent

    @classmethod
    async def connect(
        cls: Type[VoiceBoxType],
        client: GatewayBot,
        guild_id: snowflakes.Snowflake,
        channel_id: snowflakes.Snowflake,
    ) -> VoiceBoxType:
        return await client.voice.connect_to(
            guild_id,
            channel_id,
            voice_connection_type=cls,
        )

    @classmethod
    async def initialize(
        cls: Type[HikariVoicebox],
        channel_id: snowflakes.Snowflake,
        endpoint: str,
        guild_id: snowflakes.Snowflake,
        on_close: Callable[[HikariVoicebox], Awaitable[None]],
        owner: VoiceComponent,
        session_id: str,
        shard_id: int,
        token: str,
        user_id: snowflakes.Snowflake,
        **kwargs: Any,
    ) -> HikariVoicebox:
        driver = await Driver.create()
        await driver.connect(
            token=token,
            endpoint=endpoint,
            session_id=session_id,
            guild_id=guild_id,
            channel_id=channel_id,
            user_id=user_id
        )

        class_ = cls(driver)

        class_._channel_id = channel_id
        class_._guild_id = guild_id
        class_._is_alive = True
        class_._shard_id = shard_id
        class_._owner = owner

        return class_

    @property
    def channel_id(self) -> snowflakes.Snowflake:
        """Return the ID of the voice channel this voice connection is in."""
        return self._channel_id

    @property
    def guild_id(self) -> snowflakes.Snowflake:
        """Return the ID of the guild this voice connection is in."""
        return self._guild_id

    @property
    def is_alive(self) -> bool:
        """Return `builtins.True` if the connection is alive."""
        return self._is_alive

    @property
    def shard_id(self) -> int:
        """Return the ID of the shard that requested the connection."""
        return self._shard_id

    @property
    def owner(self) -> VoiceComponent:
        """Return the component that is managing this connection."""
        return self._owner

    async def disconnect(self) -> None:
        """Signal the process to shut down."""
        self._is_alive = False
        await self.driver.leave()

    async def join(self) -> None:
        """Wait for the process to halt before continuing."""

    async def notify(self, event: VoiceEvent) -> None:
        """Submit an event to the voice connection to be processed."""
