from .songbird import (
    YtdlError, FfmpegError, SongbirdError, ConsumedSourceError, CouldNotOpenFileError,
    UseAsyncConstructorError
)

__all__ = (
    "CouldNotOpenFileError",
    "FfmpegError",
    "YtdlError",
    "ConsumedSourceError",
    "UseAsyncConstructorError",
    "QueueError"
)

class QueueError(SongbirdError):
    """Queue raises an exception"""
