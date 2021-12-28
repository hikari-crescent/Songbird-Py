from __future__ import annotations

from typing import Callable, Awaitable, Any

from hikari import snowflakes, VoiceEvent
from hikari.api import VoiceComponent, VoiceConnection

from songbird import Driver, Event, Source, Config, TrackHandle, Track


class Voicebox(VoiceConnection):
    """Hikari VoiceConnection using Songbird"""

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

        self.__channel_id = channel_id
        self.__guild_id = guild_id
        self.__is_alive = True
        self.__shard_id = shard_id
        self.__owner = owner

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

    async def join(self) -> None:
        """Wait for the process to halt before continuing."""

    async def notify(self, event: VoiceEvent) -> None:
        """Submit an event to the voice connection to be processed."""

    async def leave(self) -> None:
        return await self.driver.leave()

    async def mute(self) -> None:
        return await self.driver.mute()

    async def unmute(self) -> None:
        return await self.driver.unmute()

    async def is_muted(self) -> bool:
        return await self.driver.is_muted()

    async def play_source(self, source: Source) -> TrackHandle:
        return await self.driver.play_source(source)

    async def play_only_source(self, source: Track) -> TrackHandle:
        return await self.driver.play_only_source(source)

    async def play(self, track: Track) -> None:
        return await self.driver.play(track)

    async def play_only(self, track: Track) -> None:
        return await self.driver.play_only(track)

    async def set_bitrate(self, bitrate: int) -> None:
        return await self.driver.set_bitrate(bitrate)

    async def set_bitrate_to_max(self) -> None:
        return await self.driver.set_bitrate_to_max()

    async def set_bitrate_to_auto(self) -> None:
        return await self.driver.set_bitrate_to_auto()

    async def stop(self) -> None:
        return await self.driver.stop()

    async def set_config(self, config: Config) -> None:
        return await self.driver.set_config(config)

    async def get_config(self) -> Config:
        return await self.driver.get_config()

    async def add_event(self, event: Event, call: Callable) -> None:
        return await self.driver.add_event(event, call)
