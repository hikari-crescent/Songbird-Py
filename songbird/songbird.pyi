# TODO: Docs & typing

from __future__ import annotations

from typing import (
    Any,
    Callable,
    TypeVar,
)
from dataclasses import dataclass


class SongbirdError(Exception):
    ...


class ConsumedSourceError(Exception):
    ...


class UseAsyncConstructorError(SongbirdError):
    ...


class CouldNotConnectToRTPError(SongbirdError):
    ...


class CouldNotOpenFileError(SongbirdError):
    ...


class YtdlError(SongbirdError):
    ...


class FfmpegError(SongbirdError):
    ...


DriverType = TypeVar("DriverType", bound="Driver")


class Driver:
    @staticmethod
    async def create() -> DriverType: ...
    async def connect(
        self,
        token: str,
        endpoint: str,
        session_id: str,
        guild_id: int,
        channel_id: int,
        user_id: int,
    ) -> None: ...

    async def leave(self) -> None: ...
    async def mute(self) -> None: ...
    async def unmute(self) -> None: ...
    async def is_muted(self) -> bool: ...
    async def play_source(self, source: Source) -> TrackHandle: ...
    async def play_only_source(self, source: Source) -> TrackHandle: ...
    async def play(self, source: Track) -> TrackHandle: ...
    async def play_only(self, source: Track) -> TrackHandle: ...
    async def set_bitrate(self, bitrate: int) -> None: ...
    async def set_bitrate_to_max(self) -> None: ...
    async def set_bitrate_to_auto(self) -> None: ...
    async def stop(self) -> None: ...
    async def set_config(self, config: Config) -> None: ...
    async def get_config(self) -> Config: ...
    async def add_event(
        self,
        event: Event,
        call: Callable[..., None],
    ) -> None: ...

    async def remove_all_events(self) -> None: ...


SourceType = TypeVar("SourceType", bound="Source")


class Source:
    @staticmethod
    def bytes(bytes: bytes, stereo: bool) -> SourceType: ...
    @staticmethod
    async def ffmpeg(
        filename: str,  # FIXME: Rename to `path`
        pre_input_args: tuple[str, ...] | None = None,
        args: tuple[str, ...] | None = None,
    ) -> SourceType: ...
    @staticmethod
    async def ytdl(url: str) -> SourceType: ...
    @staticmethod
    def file(url: str) -> SourceType: ...
    async def metadata(self) -> Metadata: ...
    async def stereo(self) -> bool: ...


class CryptoMode:  # TODO: Enum?
    Normal: CryptoMode
    Suffix: CryptoMode
    Lite: CryptoMode


StrategyType = TypeVar("StrategyType", bound="Strategy")


class Strategy:
    @staticmethod
    def every(duration: float) -> StrategyType: ...
    @staticmethod
    def backoff_default() -> StrategyType: ...
    @staticmethod
    def backoff(min: float, max: float, jitter: float) -> StrategyType: ...


class DecodeMode:
    Pass: DecodeMode
    Decrypt: DecodeMode
    Decode: DecodeMode


class Config:
    def __init__(self) -> None: ...
    @property
    def crypto_mode(self) -> CryptoMode: ...
    def set_crypto_mode(self, crypto_mode: CryptoMode): ...
    @property
    def decode_mode(self) -> DecodeMode: ...
    def set_decode_mode(self, decode_mode: DecodeMode): ...
    @property
    def preallocated_tracks(self) -> int: ...
    def set_preallocated_tracks(self, preallocated_tracks: int): ...
    @property
    def driver_timeout(self) -> float | None: ...
    def set_driver_timeout(self, driver_timeout: float | None): ...
    @property
    def retry_strategy(self) -> Strategy: ...
    @property
    def retry_limit(self) -> int | None: ...
    def set_driver_retry(
        self,
        strategy: Strategy,
        retry_limit: int| None,
    ): ...

    @property
    def gateway_timeout(self) -> float | None: ...
    def set_gateway_timeout(self, gateway_timeout: float | None): ...


class TrackHandle:
    def play(self) -> None: ...
    def pause(self) -> None: ...
    def stop(self) -> None: ...
    def set_volume(self, volume: float) -> None: ...
    def make_playable(self) -> None: ...
    @property
    def is_seekable(self) -> bool: ...
    def seek_time(self, position: float) -> float: ...
    def add_event(self, event: Event, call: Callable) -> None: ...
    async def get_info(self) -> TrackState: ...
    def enable_loop(self) -> None: ...
    def disable_loop(self) -> None: ...
    def loop_for(self, count: int) -> None: ...
    @property
    def uuid(self) -> str: ...
    @property
    def metadata(self) -> Metadata: ...


class TrackState:
    playing: PlayMode
    volume: float
    position: float
    play_time: float
    loops: LoopCount


class PlayMode:
    Play: PlayMode
    Pause: PlayMode
    Stop: PlayMode
    End: PlayMode

    def __eq__(self, object: Any) -> bool: ...


@dataclass
class Metadata:
    track: str | None = None
    artist: str | None = None
    date: str | None = None
    channels: int | None = None
    channel: str | None = None
    start_time: float | None = None
    duration: float | None = None
    sample_rate: int | None = None
    source_url: str | None = None
    title: str | None = None
    thumbnail: str | None = None


class LoopCount:
    loop_state: int | None


async def create_player(source: Source) -> tuple[Track, TrackHandle]: ...


class Track:
    async def play(self) -> None: ...
    async def pause(self) -> None: ...
    async def stop(self) -> None: ...
    async def playing(self) -> PlayMode: ...
    async def volume(self) -> None: ...
    async def set_volume(self, volume: float) -> None: ...
    async def position(self) -> float: ...
    async def play_time(self) -> float: ...
    async def set_loop_count(self, loops: LoopCount) -> LoopCount: ...
    async def make_playable(self) -> None: ...
    async def state(self) -> TrackState: ...
    async def seek_time(self, position: float) -> float: ...
    async def uuid(self) -> str: ...


EventType = TypeVar("EventType", bound="Event")


class Event:
    Cancel: Event
    Play: Event
    Pause: Event
    End: Event
    Loop: Event
    SpeakingStateUpdate: Event
    SpeakingUpdate: Event
    ClientDisconnect: Event
    DriverConnect: Event
    DriverReconnect: Event
    DriverDisconnect: Event

    def periodic(
        self,
        duration: float,
        phase: float | None = None
    ) -> EventType: ...

    def delayed(self, duration: float) -> EventType: ...


class SpeakingState:
    Microphone: SpeakingState
    Soundshare: SpeakingState
    Priority: SpeakingState


class Speaking:
    delay: int | None
    speaking: SpeakingState
    ssrc: int
    user_id: int | None


class SpeakingUpdateData:
    speaking: bool
    ssrc: int


class ClientConnect:
    audio_ssrc: int
    user_id: int
    video_ssrc: int


class ConnectData:
    channel_id: int
    guild_id: int
    session_id: str
    server: str
    ssrc: int


class DisconnectData:
    kind: DisconnectKind
    reason: DisconnectReason
    channel_id: int | None
    guild_id: int
    session_id: str


class DisconnectKind:
    Connect: DisconnectKind
    Reconnect: DisconnectKind
    Runtime: DisconnectKind


class DisconnectReason:
    AttemptDiscarded: DisconnectReason
    Internal: DisconnectReason
    Io: DisconnectReason
    ProtocolViolation: DisconnectReason
    TimedOut: DisconnectReason
    WsClosed: DisconnectReason
