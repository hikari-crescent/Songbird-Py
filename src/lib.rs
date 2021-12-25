use pyo3::prelude::*;

mod exceptions;
use exceptions::{
    CouldNotConnectToRTPError, CouldNotOpenFileError, FfmpegError, SongbirdError, TrackError,
    UseAsyncConstructorError, YtdlError,
};

mod config;
mod driver;
mod source;
mod track;
mod track_handle;
mod utils;

/// This module is implemented in Rust.
#[pymodule]
fn songbird(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<driver::PyDriver>()?;
    m.add_class::<source::PySource>()?;

    //Config
    m.add_class::<config::PyConfig>()?;
    m.add_class::<config::PyCryptoMode>()?;
    m.add_class::<config::PyDecodeMode>()?;
    m.add_class::<config::PyStrategy>()?;

    //track_handler
    m.add_class::<track_handle::PyPlayMode>()?;
    m.add_class::<track_handle::PyTrackHandle>()?;
    m.add_class::<track_handle::PyTrackState>()?;
    m.add_class::<track_handle::PyLoopState>()?;
    m.add_class::<track_handle::PyMetadata>()?;
    m.add_class::<track_handle::PyTrackHandle>()?;
    track::register(py, m)?;

    m.add(
        "CouldNotConnectToRTPError",
        py.get_type::<CouldNotConnectToRTPError>(),
    )?;
    m.add(
        "CouldNotOpenFileError",
        py.get_type::<CouldNotOpenFileError>(),
    )?;
    m.add("FfmpegError", py.get_type::<FfmpegError>())?;
    m.add("SongbirdError", py.get_type::<SongbirdError>())?;
    m.add("TrackError", py.get_type::<TrackError>())?;
    m.add(
        "UseAsyncConstructorError",
        py.get_type::<UseAsyncConstructorError>(),
    )?;
    m.add("YtdlError", py.get_type::<YtdlError>())?;

    Ok(())
}
