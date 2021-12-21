use pyo3::prelude::*;

mod driver;

/// This module is implemented in Rust.
#[pymodule]
fn songbird(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<driver::Driver>()?;
    m.add("CouldNotConnectToRTPError", py.get_type::<driver::CouldNotConnectToRTPError>())?;

    Ok(())
}