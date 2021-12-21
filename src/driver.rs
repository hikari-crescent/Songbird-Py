use std::sync::Arc;

use pyo3::prelude::*;
use tokio::sync::Mutex;
use songbird::Driver as _Driver;
use songbird::id::{ChannelId, GuildId, UserId};

use crate::exceptions::CouldNotConnectToRTPError;

#[pyclass]
pub struct Driver {
    driver: Arc<Mutex<Option<_Driver>>>,
}

#[pymethods]
impl Driver {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Driver {
            driver: Arc::new(Mutex::new(None)),
        })
    }

    fn make_driver<'p>(&'p self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let driver = self.driver.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut guard = driver.lock().await;
            *guard = Some(_Driver::default());
            Ok(())
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

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res = driver
                .lock()
                .await
                .as_mut()
                .unwrap()
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
}
