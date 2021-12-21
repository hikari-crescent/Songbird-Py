use pyo3::prelude::*;

mod exceptions;
use exceptions::{SongbirdError, CouldNotConnectToRTPError};

mod driver;

/// This module is implemented in Rust.
#[pymodule]
fn songbird(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<driver::Driver>()?;
    m.add("SongbirdError", py.get_type::<SongbirdError>())?;
    m.add("CouldNotConnectToRTPError", py.get_type::<CouldNotConnectToRTPError>())?;

    Ok(())
}