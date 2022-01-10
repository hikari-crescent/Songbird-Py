from __future__ import annotations

from typing import Callable

from songbird import Driver, Event, Source, Config, TrackHandle, Track


class VoiceboxBase:
    """Hikari VoiceConnection using Songbird"""

    def __init__(self, driver: Driver) -> None:
        self.driver = driver

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

    async def play_only_source(self, source: Source) -> TrackHandle:
        return await self.driver.play_only_source(source)

    async def play(self, track: Track) -> TrackHandle:
        return await self.driver.play(track)

    async def play_only(self, track: Track) -> TrackHandle:
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
