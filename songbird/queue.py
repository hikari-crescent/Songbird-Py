from __future__ import annotations

from asyncio import Event as AsyncEvent
from asyncio.tasks import ensure_future
from typing import Any, List, Optional, Union

from . import Driver, Event, Track, Source, TrackHandle


def extract_driver(driver: Any):
    if isinstance(driver, Driver):
        return driver
    return driver.driver

class Queue:
    def __init__(self, driver: Driver) -> None:
        self.driver = extract_driver(driver)
        self.queue: List[Union[Track, Source]] = []

        self.playing: Optional[TrackHandle] = None

        self.item_added: AsyncEvent = AsyncEvent()

        ensure_future(self.start())

    @classmethod
    def with_items(cls, driver: Driver, items: List[Union[Track, Source]]) -> Queue:
        q = cls(driver)
        q.queue = items
        return q

    async def start(self) -> None:
        """Starts the queue. Does not need to be called manually."""
        await self.driver.add_event(Event.End, self.__play_next)
        await self.__play_next()

    async def __play_next(self, *args) -> None:
        """Internal method. Plays the next track in the queue"""
        if not self:
            await self.item_added.wait()
            self.item_added.clear()

        next_player = self.queue.pop(0)
        if isinstance(next_player, Track):
            self.playing = await self.driver.play(next_player)
        elif isinstance(next_player, Source):
            self.playing = await self.driver.play_source(next_player)
        else:
            raise Exception(
                f"{next_player} is not a playable object. It must be of type 'Track' or"
                " 'Source'"
            )

    async def skip(self):
        """Plays the next track in the queue"""
        self.remove(self.playing)
        self.playing.stop()
        self.__play_next()

    def append(self, playable: Union[Track, Source]) -> None:
        self.item_added.set()
        self.queue.append(playable)

    def pop(self, index: int):
        return self.queue.pop(index)

    def remove(self, item: Any):
        return self.queue.remove(item)

    def insert(self, index: int, obj: Any):
        self.queue.insert(index, obj)

    def __len__(self):
        return len(self.queue)

    def __getitem__(self, index: int):
        return self.queue[index]

    def __delitem__(self, index: int):
        del self.queue[index]
