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

pub struct PySourceInner {
    /// Exists to be able to send PlayableType between threads.
    playable: PlayableType,
}

impl PySourceInner {
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

#[pyclass(name = "Source")]
pub struct PySource {
    /// Represents an object that can be turned into an input.
    /// Inputs are buffered in a Playable object due to Inputs not being thread safe.
    /// This method of creating inputs allows you to use an Input multiple times in
    /// Python, which is probably expected.
    pub source: Arc<Mutex<PySourceInner>>,
}

impl PySource {
    fn new(playable: PlayableType) -> Self {
        Self {
            source: Arc::from(Mutex::from(PySourceInner::new(playable))),
        }
    }
}

#[pymethods]
impl PySource {
    #[staticmethod]
    fn ytdl<'p>(url: String) -> PyResult<Self> {
        //! Use youtube dl to play a video from a URL
        //!
        //! # Example
        //! ```python
        //! await driver.play(Playable.ytdl("https://www.youtube.com/watch?v=n5n7CSGPzqw"))
        //! ```
        Ok(Self::new(PlayableType::Ytdl(url)))
    }

    #[staticmethod]
    fn bytes<'p>(bytes: Vec<u8>, stereo: bool) -> PyResult<Self> {
        Ok(Self::new(PlayableType::Bytes(bytes, stereo)))
    }

    #[staticmethod]
    fn file<'p>(filepath: String, stereo: bool) -> PyResult<Self> {
        //! This plays the bytes from the file, DO NOT use for mp3s, etc
        //! ffmpeg should be used instead.
        Ok(Self::new(PlayableType::File(filepath, stereo)))
    }

    #[staticmethod]
    fn ffmpeg<'p>(filepath: String) -> PyResult<Self> {
        //! Function used to play most audio formats
        //!
        //! # Example
        //! ```python
        //! await driver.play(Playable.ffmpeg("song.mp3"))
        Ok(Self::new(PlayableType::Ffmpeg(filepath)))
    }
}
