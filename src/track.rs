use std::time::Duration;

use pyo3::prelude::*;
use songbird::tracks::Track;

use crate::source::PySource;
use crate::track_handle::{handle_track_result, PyLoopState, PyPlayMode, PyTrackState};

#[pyfunction]
#[pyo3(name = "create_player")]
pub fn py_create_player<'p>(py: Python<'p>, source: &'p PySource) -> PyResult<&'p PyAny> {
    let source = source.source.clone();
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let pyinput = source.lock().await;
        let source = pyinput.get_input().await?;
        let (track, _) = songbird::create_player(source);
        Ok(PyTack { track })
    })
}

#[pyclass]
pub struct PyTack {
    pub track: Track,
}

#[pymethods(name = "Track")]
impl PyTack {
    #[pyo3(text_signature = "($self)")]
    fn play(&mut self) -> () {
        self.track.play();
    }
    #[pyo3(text_signature = "($self)")]
    fn pause(&mut self) -> () {
        self.track.pause();
    }
    #[pyo3(text_signature = "($self)")]
    fn stop(&mut self) -> () {
        self.track.stop();
    }
    #[getter]
    fn playing(&mut self) -> PyPlayMode {
        PyPlayMode::from(self.track.playing())
    }
    #[getter]
    fn get_volume(&mut self) -> f32 {
        self.track.volume()
    }
    #[pyo3(text_signature = "($self)")]
    fn set_volume(&mut self, volume: f32) -> () {
        self.track.set_volume(volume);
    }
    #[getter]
    fn position(&mut self) -> f64 {
        self.track.position().as_secs_f64()
    }
    #[getter]
    fn play_time(&mut self) -> f64 {
        self.track.play_time().as_secs_f64()
    }
    #[pyo3(text_signature = "($self, loops)")]
    fn set_loops(&mut self, loops: &PyLoopState) -> PyResult<()> {
        handle_track_result(self.track.set_loops(loops.as_songbird_loop_state()))
    }
    #[pyo3(text_signature = "($self)")]
    fn make_playable(&mut self) -> () {
        self.track.make_playable();
    }
    #[getter]
    fn state(&self) -> PyTrackState {
        PyTrackState::from(self.track.state())
    }
    #[pyo3(text_signature = "($self)")]
    fn seek_time(&mut self, position: f64) -> PyResult<f64> {
        match handle_track_result(self.track.seek_time(Duration::from_secs_f64(position))) {
            Ok(dur) => Ok(dur.as_secs_f64()),
            Err(err) => Err(err),
        }
    }
    #[getter]
    fn uuid(&self) -> String {
        self.track.uuid().to_string()
    }
}
