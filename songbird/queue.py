from __future__ import annotations

from asyncio import Event as AsyncEvent
from asyncio.tasks import ensure_future
from typing import Any, List, Optional, Union

from .songbird import Driver, Event, Track, Source, TrackHandle


def extract_driver(driver: Any):
    if isinstance(driver, Driver):
        return driver
    return driver.driver


class Queue(list):
    """
    Parameters
    ----------
    driver : :class:`~songbird.songbird.Driver`
        The driver to control.
    
    Attributes
    ----------
    current_track_handle : Optional[:class:`~songbird.songbird.TrackHandle`]
        The TrackHandle for the currently playing song.
    running : bool
        If the Queue is running or not. This will be :data:`True` if the Queue isn't
        stopped.
    """
    def __init__(self, driver: Driver) -> None:
        self.driver = extract_driver(driver)
        self.current_track_handle: Optional[TrackHandle] = None
        self.running = True

        self.item_added: AsyncEvent = AsyncEvent()

        ensure_future(self.start())

    @classmethod
    def with_items(cls, driver: Driver, items: List[Union[Track, Source]]) -> Queue:
        q = cls(driver)
        q.extend(items)
        return q

    async def start(self) -> None:
        """Starts the queue. Does not need to be called manually."""
        await self.driver.add_event(Event.End, self.__play_next)
        await self.__play_next()

    async def stop(self) -> None:
        """Stops the queue from running. Does not stop the currently playing song."""
        self.running = False

    async def __play_next(self, *args) -> None:
        """Internal method. Plays the next track in the queue"""
        if not self.running:
            return

        if not self:
            await self.item_added.wait()
            self.item_added.clear()

        next_player = self.pop(0)
        if isinstance(next_player, Track):
            self.current_track_handle = await self.driver.play(next_player)
        elif isinstance(next_player, Source):
            self.current_track_handle = await self.driver.play_source(next_player)
        else:
            raise Exception(
                f"{next_player} is not a playable object. It must be of type 'Track' or"
                " 'Source'"
            )

    def skip(self):
        """Plays the next track in the queue"""
        self.current_track_handle.stop()
        ensure_future(self.__play_next())
