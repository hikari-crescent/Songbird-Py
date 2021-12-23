use std::fs::File;
use std::sync::Arc;

use tokio::sync::Mutex;

use pyo3::prelude::*;
use songbird::input::{Input, Reader};

use crate::exceptions::{CouldNotOpenFileError, FfmpegError, YtdlError};

enum PlayableType {
    Bytes(Vec<u8>, bool),
    File(String, bool),
    Ffmpeg(String),
    Ytdl(String),
}

pub struct PyPlayableInner {
    playable: PlayableType,
}

impl PyPlayableInner {
    fn new(playable: PlayableType) -> Self {
        Self { playable }
    }

    pub async fn get_input(&self) -> Result<Input, PyErr> {
        match &self.playable {
            PlayableType::Ytdl(url) => match songbird::ytdl(url).await {
                Ok(res) => Ok(res),
                Err(err) => Err(YtdlError::new_err(format!("{:?}", err))),
            },
            PlayableType::Ffmpeg(filepath) => match songbird::ffmpeg(filepath).await {
                Ok(res) => Ok(res),
                Err(err) => Err(FfmpegError::new_err(format!("{:?}", err))),
            },
            PlayableType::File(filepath, stereo) => match File::open(filepath) {
                Ok(res) => Ok(Input::float_pcm(*stereo, Reader::from_file(res))),
                Err(err) => Err(CouldNotOpenFileError::new_err(format!("{:?}", err))),
            },
            PlayableType::Bytes(bytes, stereo) => Ok(Input::float_pcm(
                *stereo,
                Reader::from_memory(bytes.to_vec()),
            )),
        }
    }
}

#[pyclass(name = "Playable")]
pub struct PyPlayable {
    pub source: Arc<Mutex<PyPlayableInner>>,
}

impl PyPlayable {
    fn new(playable: PlayableType) -> Self {
        Self {
            source: Arc::from(Mutex::from(PyPlayableInner::new(playable))),
        }
    }
}

#[pymethods]
impl PyPlayable {
    //! Represents an object that can be turned into an input.
    //! Inputs are buffered in a Playable object due to Inputs not being thread safe.
    //! This method of creating inputs allows you to use an Input multiple times in
    //! Python, which is probably expected.
    #[staticmethod]
    fn ytdl<'p>(url: String) -> PyResult<PyPlayable> {
        Ok(Self::new(PlayableType::Ytdl(url)))
    }

    #[staticmethod]
    fn bytes<'p>(bytes: Vec<u8>, stereo: bool) -> PyResult<PyPlayable> {
        Ok(Self::new(PlayableType::Bytes(bytes, stereo)))
    }

    #[staticmethod]
    fn file<'p>(filepath: String, stereo: bool) -> PyResult<PyPlayable> {
        Ok(Self::new(PlayableType::File(filepath, stereo)))
    }

    #[staticmethod]
    fn ffmpeg<'p>(filepath: String) -> PyResult<PyPlayable> {
        Ok(Self::new(PlayableType::Ffmpeg(filepath)))
    }
}
