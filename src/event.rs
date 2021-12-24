use pyo3::prelude::*;

use songbird::events::Event;

#[pyclass]
pub struct PyEvent {
    pub event: Event
}
