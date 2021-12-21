use pyo3::prelude::*;
use songbird::id;

use fast_async_mutex::rwlock::RwLock;
use pyo3::create_exception;
use std::sync::Arc;

create_exception!(
    module,
    CouldNotConnectToRTPError,
    pyo3::exceptions::PyException
);

#[pyclass]
pub struct Driver {
    driver: Arc<RwLock<Option<songbird::Driver>>>,
}

#[pymethods]
impl Driver {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Driver {
            driver: Arc::new(RwLock::new(None)),
        })
    }

    fn make_driver<'p>(&'p self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let driver = self.driver.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut guard = driver.write_owned().await;
            *guard = Some(songbird::Driver::default());
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
                .write()
                .await
                .as_mut()
                .unwrap()
                .connect(songbird::ConnectionInfo {
                    channel_id: Some(id::ChannelId::from(channel_id)),
                    endpoint: endpoint,
                    guild_id: id::GuildId::from(guild_id),
                    session_id: session_id,
                    token: token,
                    user_id: id::UserId::from(user_id),
                })
                .await;

            match res {
                Err(err) => Err(CouldNotConnectToRTPError::new_err(
                    format!("{:?}", err),
                )),
                Ok(_) => Ok(()),
            }
        })
    }
}
