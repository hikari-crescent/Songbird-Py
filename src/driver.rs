use std::sync::Arc;

use pyo3::prelude::*;
use songbird::id::{ChannelId, GuildId, UserId};
use songbird::Driver;
use tokio::sync::Mutex;

use crate::exceptions::{CouldNotConnectToRTPError, UseAsyncConstructorError};
use crate::input::PyPlayable;

#[pyclass(name = "Driver")]
pub struct PyDriver {
    driver: Arc<Mutex<Driver>>,
}

#[pymethods]
impl PyDriver {
    #[new]
    fn new() -> PyResult<Self> {
        Err(UseAsyncConstructorError::new_err(
            "`await Driver.create()` should be used to construct this class.",
        ))
    }

    #[staticmethod]
    fn create<'p>(py: Python<'p>) -> PyResult<&'p PyAny> {
        pyo3_asyncio::tokio::future_into_py(py, async move {
            Ok(PyDriver {
                driver: Arc::new(Mutex::new(Driver::default())),
            })
        })
    }

    fn connect<'p>(
        &'p self,
        py: Python<'p>,
        token: String,
        endpoint: String,
        session_id: String,
        guild_id: u64,
        channel_id: u64,
        user_id: u64,
    ) -> PyResult<&'p PyAny> {
        let driver = self.driver.clone();

        let endpoint = endpoint.replace("wss://", "");

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res = driver
                .lock()
                .await
                .connect(songbird::ConnectionInfo {
                    channel_id: Some(ChannelId::from(channel_id)),
                    endpoint: endpoint,
                    guild_id: GuildId::from(guild_id),
                    session_id: session_id,
                    token: token,
                    user_id: UserId::from(user_id),
                })
                .await;

            match res {
                Err(err) => Err(CouldNotConnectToRTPError::new_err(format!("{:?}", err))),
                Ok(_) => Ok(()),
            }
        })
    }

    fn leave<'p>(&'p self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let driver = self.driver.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            driver.lock().await.leave();
            Ok(())
        })
    }

    fn play<'p>(&'p self, py: Python<'p>, reader: &'p PyPlayable) -> PyResult<&'p PyAny> {
        let driver = self.driver.clone();
        let source = reader.source.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let source = source.lock().await.get_input().await;
            if let Err(err) = source {
                return Err(err);
            }

            driver.lock().await.play_source(source.unwrap());
            Ok(())
        })
    }
}
