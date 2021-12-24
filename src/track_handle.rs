use std::time::Duration;

use pyo3::prelude::*;
use songbird::tracks::{TrackHandle, TrackResult};

use crate::exceptions::TrackError;

fn handle_track_result<'p, T>(res: TrackResult<T>) -> Result<T, PyErr> {
    match res {
        Ok(t) => Ok(t),
        Err(err) => Err(TrackError::new_err(format!("{:?}", err))),
    }
}

#[pyclass(name = "TrackHandle")]
pub struct PyTrackHandle {
    track_handle: TrackHandle,
}

impl PyTrackHandle {
    pub fn from(track_handle: TrackHandle) -> Self {
        Self { track_handle }
    }
}

#[pymethods]
impl PyTrackHandle {
    #[pyo3(text_signature = "($self)")]
    fn play(&self) -> PyResult<()> {
        handle_track_result(self.track_handle.play())
    }
    #[pyo3(text_signature = "($self)")]
    fn pause(&self) -> PyResult<()> {
        handle_track_result(self.track_handle.pause())
    }
    #[pyo3(text_signature = "($self)")]
    fn stop(&self) -> PyResult<()> {
        handle_track_result(self.track_handle.pause())
    }
    #[pyo3(text_signature = "($self, volume)")]
    fn set_volume(&self, volume: f32) -> PyResult<()> {
        handle_track_result(self.track_handle.set_volume(volume))
    }
    #[pyo3(text_signature = "($self)")]
    fn make_playable(&self) -> PyResult<()> {
        handle_track_result(self.track_handle.make_playable())
    }
    #[pyo3(text_signature = "($self)")]
    fn is_seekable(&self) -> bool {
        self.track_handle.is_seekable()
    }
    #[pyo3(text_signature = "($self)")]
    fn seek_time(&self, position: f64) -> PyResult<()> {
        handle_track_result(
            self.track_handle
                .seek_time(Duration::from_secs_f64(position)),
        )
    }
}
