use pyo3::create_exception;

// Base Exception for all songbird errors
create_exception!(module, SongbirdError, pyo3::exceptions::PyException);

create_exception!(module, CouldNotConnectToRTPError, SongbirdError);
