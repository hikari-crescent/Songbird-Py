use pyo3::prelude::*;

mod exceptions;
use exceptions::{
    CouldNotConnectToRTPError, CouldNotOpenFileError, FfmpegError, SongbirdError,
    UseAsyncConstructorError, YtdlError,
};

mod driver;
mod input;

/// This module is implemented in Rust.
#[pymodule]
fn songbird(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<driver::PyDriver>()?;
    m.add_class::<input::PyPlayable>()?;
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
    m.add(
        "UseAsyncConstructorError",
        py.get_type::<UseAsyncConstructorError>(),
    )?;
    m.add("YtdlError", py.get_type::<YtdlError>())?;

    Ok(())
}
