use pyo3::create_exception;

// Base Exception for all songbird errors
create_exception!(module, SongbirdError, pyo3::exceptions::PyException);

create_exception!(module, UseAsyncConstructorError, SongbirdError);
create_exception!(module, CouldNotConnectToRTPError, SongbirdError);
create_exception!(module, ConsumedSourceError, SongbirdError);
create_exception!(module, CouldNotOpenFileError, SongbirdError);
create_exception!(module, YtdlError, SongbirdError);
create_exception!(module, FfmpegError, SongbirdError);
create_exception!(module, TrackError, SongbirdError);
