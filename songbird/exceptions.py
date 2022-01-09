from .songbird import (
    YtdlError, FfmpegError, SongbirdError, ConsumedSourceError, CouldNotOpenFileError,
    UseAsyncConstructorError
)

__all__ = (
    "CouldNotOpenFileError",
    "FfmpegError",
    "YtdlError",
    "ConsumedSourceError",
    "UseAsyncConstructorError"
)

class QueueError(SongbirdError):
    """Queue raises an exception"""
