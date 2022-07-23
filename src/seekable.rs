use std::mem;

use pyo3::prelude::*;
use songbird::input::{cached::Compressed, Restartable};

use crate::config::PyBitrate;
use crate::exceptions::{
    ConsumedSourceError, CouldNotConstructError, FfmpegError, UseAsyncConstructorError, YtdlError,
};
use crate::source::PySource;

#[pyclass(name = "RestartableSource")]
pub struct PyRestartableSource {
    restartable: Option<Restartable>,
}

impl From<Restartable> for PyRestartableSource {
    fn from(restartable: Restartable) -> Self {
        Self {
            restartable: Some(restartable),
        }
    }
}

#[pymethods]
impl PyRestartableSource {
    /// Convert the RestartableSource into a Source
    ///
    /// Example
    /// .. code-block:: python
    ///
    ///     import songbird
    ///     restartable: songbird.RestartableSource
    ///     driver: songbird.Driver
    ///     driver.play_source(restartable.into_source())
    fn into_source(&mut self) -> Result<PySource, PyErr> {
        let maybe_restartable = mem::take(&mut self.restartable);
        if let Some(restartable) = maybe_restartable {
            Ok(PySource::from(restartable.into()))
        } else {
            Err(ConsumedSourceError::new_err(
                "RestartableSource already converted to source.",
            ))
        }
    }

    /// Create a seekable source from a URL. The cost of seeking is very high.
    #[staticmethod]
    fn ytdl<'p>(py: Python, url: String, lazy: bool) -> PyResult<&PyAny> {
        pyo3_asyncio::tokio::future_into_py(py, async move {
            match Restartable::ytdl(url, lazy).await {
                Ok(res) => Ok(Self::from(res)),
                Err(err) => Err(YtdlError::new_err(format!("{:?}", err))),
            }
        })
    }

    /// Create a seekable source from a file with ffmpeg.
    #[staticmethod]
    fn ffmpeg<'p>(py: Python, filename: String, lazy: bool) -> PyResult<&PyAny> {
        pyo3_asyncio::tokio::future_into_py(py, async move {
            match Restartable::ffmpeg(filename, lazy).await {
                Ok(res) => Ok(Self::from(res)),
                Err(err) => Err(FfmpegError::new_err(format!("{:?}", err))),
            }
        })
    }
}

#[pyclass(name = "CompressedSource")]
pub struct PyCompressedSource {
    compressed: Option<Compressed>,
}

impl From<Compressed> for PyCompressedSource {
    fn from(memory: Compressed) -> Self {
        Self {
            compressed: Some(memory),
        }
    }
}

#[pymethods]
impl PyCompressedSource {
    #[new]
    fn new() -> PyResult<Self> {
        Err(UseAsyncConstructorError::new_err(
            "Use `CompressedSource.from_source` to create a `CompressedSource` object.",
        ))
    }

    /// Convert the MemorySource into a Source
    fn into_source(&mut self) -> Result<PySource, PyErr> {
        let maybe_compressed = mem::take(&mut self.compressed);
        if let Some(compressed) = maybe_compressed {
            Ok(PySource::from(compressed.into()))
        } else {
            Err(ConsumedSourceError::new_err(
                "MemorySource already converted to source.",
            ))
        }
    }

    #[staticmethod]
    fn from_source<'p>(
        py: Python<'p>,
        input: &PySource,
        bitrate: &PyBitrate,
    ) -> PyResult<&'p PyAny> {
        let source = input.source.clone();
        let bitrate = bitrate.bitrate;

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut source = source.lock().await;
            let old = mem::take(&mut *source);

            match Compressed::new(old.unwrap(), bitrate) {
                Ok(c) => Ok(Self {
                    compressed: Some(c),
                }),
                Err(reason) => Err(CouldNotConstructError::new_err(reason.to_string())),
            }
        })
    }
}
