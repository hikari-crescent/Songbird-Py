use std::fs::File;
use std::sync::Arc;

use tokio::sync::Mutex;

use pyo3::prelude::*;
use songbird::input::{Input, Reader};

use crate::exceptions::{ConsumedSourceError, CouldNotOpenFileError, FfmpegError, YtdlError};

#[pyclass(name = "Source")]
pub struct PySource {
    /// Represents an object that can be turned into an input.
    /// Inputs are buffered in a Playable object due to Inputs not being thread safe.
    /// This method of creating inputs allows you to use an Input multiple times in
    /// Python, which is probably expected.
    pub source: Arc<Mutex<Option<Input>>>,
    consumed: bool,
}

impl PySource {
    fn from(input: Input) -> Self {
        Self {
            source: Arc::from(Mutex::from(Some(input))),
            consumed: false,
        }
    }

    pub fn raise_if_consumed(&mut self) -> Result<(), PyErr> {
        if self.consumed {
            Err(ConsumedSourceError::new_err(concat!(
                "Source object has already been used! Sources can only create a track",
                " or be played in a driver once."
            )))
        } else {
            self.mark_consumed();
            Ok(())
        }
    }

    pub fn mark_consumed(&mut self) -> () {
        self.consumed = true
    }
}

#[pymethods]
impl PySource {
    /// Use youtube dl to play a video from a URL
    ///
    /// # Example
    /// .. code-block:: python
    ///
    ///     await driver.play(Source.ytdl("https://www.youtube.com/watch?v=n5n7CSGPzqw"))
    #[staticmethod]
    fn ytdl<'p>(py: Python, url: String) -> PyResult<&PyAny> {
        pyo3_asyncio::tokio::future_into_py(py, async move {
            match songbird::ytdl(url).await {
                Ok(res) => Ok(Self::from(res)),
                Err(err) => Err(YtdlError::new_err(format!("{:?}", err))),
            }
        })
    }

    /// Create a source from bytes.
    #[staticmethod]
    fn bytes<'p>(bytes: Vec<u8>, stereo: bool) -> PyResult<Self> {
        Ok(Self::from(Input::float_pcm(
            stereo,
            Reader::from_memory(bytes.to_vec()),
        )))
    }

    /// This plays the bytes from the file, DO NOT use for mp3s, etc
    /// ffmpeg should be used instead.
    #[staticmethod]
    fn file<'p>(filepath: String, stereo: bool) -> PyResult<Self> {
        match File::open(filepath) {
            Ok(res) => Ok(Self::from(Input::float_pcm(stereo, Reader::from_file(res)))),
            Err(err) => Err(CouldNotOpenFileError::new_err(format!("{:?}", err))),
        }
    }

    /// Function used to play most audio formats
    ///
    /// .. code-block:: python
    ///
    ///     await driver.play(Source.ffmpeg("song.mp3"))
    #[staticmethod]
    fn ffmpeg<'p>(py: Python, filepath: String) -> PyResult<&PyAny> {
        pyo3_asyncio::tokio::future_into_py(py, async move {
            match songbird::ffmpeg(filepath).await {
                Ok(res) => Ok(Self::from(res)),
                Err(err) => Err(FfmpegError::new_err(format!("{:?}", err))),
            }
        })
    }
}
