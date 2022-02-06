from .songbird import (
    YtdlError,
    FfmpegError,
    SongbirdError,
    ConsumedSourceError,
    CouldNotOpenFileError,
    UseAsyncConstructorError,
)


class QueueError(SongbirdError):
    """Queue raises an exception"""


__all__ = [
    "CouldNotOpenFileError",
    "FfmpegError",
    "YtdlError",
    "ConsumedSourceError",
    "UseAsyncConstructorError",
    "QueueError",
]

