from __future__ import annotations

import typing
from asyncio import Event as AsyncEvent
from asyncio.tasks import ensure_future
from logging import Logger

from songbird.exceptions import QueueError

from .songbird import Driver, Event, Track, Source, TrackHandle

_log = Logger(__name__)


def extract_driver(driver: typing.Any):
    if isinstance(driver, Driver):
        return driver
    return driver.driver


T = typing.TypeVar("T")


class Queue(list):
    """
    Parameters
    ----------
    driver : :class:`~songbird.songbird.Driver`
        The driver to control.

    Attributes
    ----------
    track_handle : Optional[:class:`~songbird.songbird.TrackHandle`]
        The TrackHandle for the currently playing song.
    running : bool
        If the Queue is running or not. This will be :data:`True` if the Queue isn't
        stopped.
    on_next : Callable[[Driver, Any], Awaitable[:data:`None`]], default=None
        Function called when the next song in the queue starts playing.
    on_fail : Callable[[Driver, Any], Awaitable[:data:`None`]], default=None
        Function called when a song failed to play.
    """

    def __init__(
        self,
        driver: Driver,
        on_next: typing.Callable[[Driver, typing.Any], typing.Awaitable[None]] | None = None,
        on_fail: typing.Callable[[Driver, typing.Any], typing.Awaitable[None]] | None = None
    ) -> None:
        super().__init__()
        self.driver = extract_driver(driver)
        self.track_handle: TrackHandle | None = None
        self.running = False

        self.on_next = on_next
        self.on_fail = on_fail

        self.item_added: AsyncEvent = AsyncEvent()

        ensure_future(self.start())

    @classmethod
    def with_items(
        cls,
        driver: Driver,
        items: list[Track | Source],
        **kwargs: typing.Any,
    ) -> Queue:
        q = cls(driver, **kwargs)
        q.extend(items)
        return q

    async def start(self) -> None:
        """Starts the queue. Does not need to be called manually."""
        if self.running:
            return

        self.running = True

        await self.driver.add_event(Event.End, self._play_next)
        await self._play_next()

    async def stop(self) -> None:
        """Stops the queue from running. Does not stop the currently playing song."""
        self.running = False

    def append(self, __object: T) -> None:
        self.item_added.set()
        return super().append(__object)

    def extend(self, __iterable: typing.Iterable[T]) -> None:
        self.item_added.set()
        return super().extend(__iterable)

    def insert(self, __index: typing.SupportsIndex, __object: T) -> None:
        self.item_added.set()
        return super().insert(__index, __object)

    def __add__(self, __x: list[T]) -> list[T]:
        self.item_added.set()
        return super().__add__(__x)

    def __iadd__(self: Queue, __x: typing.Iterable[T]) -> Queue:
        self.item_added.set()
        return super().__iadd__(__x)

    async def _play_next(self, *_args: typing.Any) -> None:
        """Internal method. Plays the next track in the queue"""
        if not self.running:
            return

        while True:
            if not self:
                await self.item_added.wait()

            self.item_added.clear()

            next_player = self.pop(0)

            # This allows players to be added to the queue without being activated
            # immediatly helps save memory when adding an entire playlist.
            if isinstance(next_player, typing.Awaitable):
                try:
                    next_player = await next_player
                    break
                except Exception as e:
                    _log.warning(
                        f"Failed to play song because of exception `{e}`."
                        " Skipping to next song."
                    )

                    if self.on_fail:
                        ensure_future(self.on_fail(self.driver, next_player))
            else:
                break

        if isinstance(next_player, Track):
            self.track_handle = await self.driver.play(next_player)
        elif isinstance(next_player, Source):
            self.track_handle = await self.driver.play_source(next_player)
        else:
            raise QueueError(
                f"{next_player} is not a playable object. It must be of type 'Track' or"
                " 'Source'"
            )

        if self.on_next:
            self.on_next(self.driver, self.track_handle)

    def skip(self) -> None:
        """Plays the next track in the queue"""
        if not self.track_handle:
            raise QueueError("No track is playing")

        self.track_handle.stop()
        self.track_handle = None
        ensure_future(self._play_next())
