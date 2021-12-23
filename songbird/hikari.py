from __future__ import annotations

from typing import Callable, Awaitable, Any

from hikari import snowflakes, VoiceEvent
from hikari.api import VoiceComponent, VoiceConnection, shard

from songbird import Driver, Playable


class Voicebox(VoiceConnection):
    def __init__(self, driver: Driver) -> None:
        self.driver = driver

    @classmethod
    async def initialize(
        cls: Voicebox,
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

        self = Voicebox(driver)

        self._channel_id = channel_id
        self._guild_id = guild_id
        self._is_alive = True
        self._shard_id = shard_id
        self._owner = owner

        return self

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
        await self.driver.leave()

    async def join(self) -> None:
        """Wait for the process to halt before continuing."""

    async def notify(self, event: VoiceEvent) -> None:
        """Submit an event to the voice connection to be processed."""

    async def play(self, playable: Playable):
        await self.driver.play(playable)
