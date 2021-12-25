use std::sync::Arc;
use std::time::Duration;

use pyo3::prelude::*;
use songbird::tracks::Track;
use tokio::sync::Mutex;

use crate::source::PySource;
use crate::track_handle::{
    handle_track_result, PyLoopState, PyPlayMode, PyTrackHandle, PyTrackState,
};

#[pyfunction]
#[pyo3(name = "create_player")]
pub fn py_create_player<'p>(py: Python<'p>, source: &'p PySource) -> PyResult<&'p PyAny> {
    let source = source.source.clone();
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let pyinput = source.lock().await;
        let source = pyinput.get_input().await?;
        let (track, handle) = songbird::create_player(source);

        Ok((
            PyTrack {
                track: Arc::from(Mutex::from(Some(track))),
            },
            PyTrackHandle::from(handle),
        ))
    })
}

#[allow(unused_variables)]
pub(crate) fn register(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_create_player, m)?)?;
    Ok(())
}

#[pyclass]
pub struct PyTrack {
    pub track: Arc<Mutex<Option<Track>>>,
}

#[pymethods(name = "Track")]
impl PyTrack {
    #[pyo3(text_signature = "($self)")]
    fn play<'p>(&'p mut self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let track = self.track.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            track.lock().await.as_mut().unwrap().play();
            Ok(())
        })
    }
    #[pyo3(text_signature = "($self)")]
    fn pause<'p>(&'p mut self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let track = self.track.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            track.lock().await.as_mut().unwrap().pause();
            Ok(())
        })
    }
    #[pyo3(text_signature = "($self)")]
    fn stop<'p>(&'p mut self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let track = self.track.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            track.lock().await.as_mut().unwrap().stop();
            Ok(())
        })
    }
    #[pyo3(text_signature = "($self)")]
    fn playing<'p>(&'p mut self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let track = self.track.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let play_mode = track.lock().await.as_mut().unwrap().playing();
            Ok(PyPlayMode::from(play_mode))
        })
    }
    #[pyo3(text_signature = "($self)")]
    fn volume<'p>(&'p mut self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let track = self.track.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            Ok(track.lock().await.as_mut().unwrap().volume())
        })
    }
    #[pyo3(text_signature = "($self, volume)")]
    fn set_volume<'p>(&'p mut self, py: Python<'p>, volume: f32) -> PyResult<&'p PyAny> {
        let track = self.track.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            track.lock().await.as_mut().unwrap().set_volume(volume);
            Ok(())
        })
    }
    #[pyo3(text_signature = "($self)")]
    fn position<'p>(&mut self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let track = self.track.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            Ok(track
                .lock()
                .await
                .as_mut()
                .unwrap()
                .position()
                .as_secs_f64())
        })
    }
    #[pyo3(text_signature = "($self)")]
    fn play_time<'p>(&mut self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let track = self.track.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            Ok(track
                .lock()
                .await
                .as_mut()
                .unwrap()
                .play_time()
                .as_secs_f64())
        })
    }
    #[pyo3(text_signature = "($self, loops)")]
    fn set_loop_count<'p>(&mut self, py: Python<'p>, loops: Option<usize>) -> PyResult<&'p PyAny> {
        let track = self.track.clone();
        let loops = loops.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            handle_track_result(
                track
                    .lock()
                    .await
                    .as_mut()
                    .unwrap()
                    .set_loops(PyLoopState::from_usize(loops).as_songbird_loop_state()),
            )
        })
    }
    #[pyo3(text_signature = "($self)")]
    fn make_playable<'p>(&mut self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let track = self.track.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            track.lock().await.as_mut().unwrap().make_playable();
            Ok(())
        })
    }
    #[pyo3(text_signature = "($self)")]
    fn state<'p>(&mut self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let track = self.track.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            Ok(PyTrackState::from(
                track.lock().await.as_mut().unwrap().state(),
            ))
        })
    }
    #[pyo3(text_signature = "($self)")]
    fn seek_time<'p>(&mut self, py: Python<'p>, position: f64) -> PyResult<&'p PyAny> {
        let track = self.track.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            match handle_track_result(
                track
                    .lock()
                    .await
                    .as_mut()
                    .unwrap()
                    .seek_time(Duration::from_secs_f64(position)),
            ) {
                Ok(dur) => Ok(dur.as_secs_f64()),
                Err(err) => Err(err),
            }
        })
    }
    #[pyo3(text_signature = "($self)")]
    fn uuid<'p>(&'p mut self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let track = self.track.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            Ok(track.lock().await.as_mut().unwrap().uuid().to_string())
        })
    }
}
