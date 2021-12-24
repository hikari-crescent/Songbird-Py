use std::time::Duration;

use pyo3::prelude::*;
use songbird::driver::{
    retry::{ExponentialBackoff, Retry, Strategy},
    CryptoMode, DecodeMode,
};
use songbird::Config;

use crate::utils::unwrap_f64_to_duration;

#[pyclass(name = "CryptoMode")]
pub struct PyCryptoMode {
    crypto_mode: CryptoMode,
}

impl PyCryptoMode {
    fn from(crypto_mode: CryptoMode) -> Self {
        Self { crypto_mode }
    }
}

#[allow(non_snake_case)]
#[pymethods]
impl PyCryptoMode {
    #[classattr]
    fn Normal() -> Self {
        Self::from(CryptoMode::Normal)
    }
    #[classattr]
    fn Suffix() -> Self {
        Self::from(CryptoMode::Suffix)
    }
    #[classattr]
    fn Lite() -> Self {
        Self::from(CryptoMode::Lite)
    }
}

#[pyclass(name = "Strategy")]
pub struct PyStrategy {
    strategy: Strategy,
}

impl PyStrategy {
    fn from(strategy: Strategy) -> Self {
        Self { strategy }
    }
}

#[pymethods]
impl PyStrategy {
    #[staticmethod]
    #[pyo3(text_signature = "(duration)")]
    fn every(duration: f64) -> Self {
        Self::from(Strategy::Every(Duration::from_secs_f64(duration)))
    }

    #[staticmethod]
    #[pyo3(text_signature = "()")]
    fn backoff_default() -> Self {
        Self::from(Strategy::Backoff(ExponentialBackoff::default()))
    }

    #[staticmethod]
    #[pyo3(text_signature = "(min, max, jitter)")]
    fn backoff(min: f64, max: f64, jitter: f32) -> Self {
        Self::from(Strategy::Backoff(ExponentialBackoff {
            min: Duration::from_secs_f64(min),
            max: Duration::from_secs_f64(max),
            jitter,
        }))
    }
}

#[pyclass(name = "DecodeMode")]
pub struct PyDecodeMode {
    decode_mode: DecodeMode,
}

impl PyDecodeMode {
    fn from(decode_mode: DecodeMode) -> Self {
        Self { decode_mode }
    }
}

#[allow(non_snake_case)]
#[pymethods]
impl PyDecodeMode {
    #[classattr]
    fn Pass() -> Self {
        Self::from(DecodeMode::Pass)
    }
    #[classattr]
    fn Decrypt() -> Self {
        Self::from(DecodeMode::Decrypt)
    }
    #[classattr]
    fn Decode() -> Self {
        Self::from(DecodeMode::Decode)
    }
}

#[pyclass(name = "Config")]
#[pyo3(text_signature = "(/)")]
pub struct PyConfig {
    pub config: Config,
}

#[pymethods]
impl PyConfig {
    #[new]
    fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    #[pyo3(text_signature = "($self, crypto_mode)")]
    fn set_crypto_mode(&mut self, crypto_mode: &PyCryptoMode) {
        let config = self.config.clone();
        self.config = config.crypto_mode(crypto_mode.crypto_mode)
    }

    #[pyo3(text_signature = "($self, decode_mode)")]
    fn set_decode_mode(&mut self, decode_mode: &PyDecodeMode) {
        let config = self.config.clone();
        self.config = config.decode_mode(decode_mode.decode_mode)
    }

    #[pyo3(text_signature = "($self, preallocated_tracks)")]
    fn set_preallocated_tracks(&mut self, preallocated_tracks: usize) {
        let config = self.config.clone();
        self.config = config.preallocated_tracks(preallocated_tracks)
    }

    #[pyo3(text_signature = "($self, driver_timeout)")]
    fn set_driver_timeout(&mut self, driver_timeout: Option<f64>) {
        let config = self.config.clone();
        self.config = config.driver_timeout(unwrap_f64_to_duration(driver_timeout))
    }

    #[pyo3(text_signature = "($self, driver_retry)")]
    fn set_driver_retry(&mut self, strategy: &PyStrategy, retry_limit: Option<usize>) {
        let config = self.config.clone();
        self.config = config.driver_retry(Retry {
            strategy: strategy.strategy,
            retry_limit,
        })
    }

    #[pyo3(text_signature = "($self, crypto_mode)")]
    fn set_gateway_timeout(&mut self, gateway_timeout: Option<f64>) {
        let config = self.config.clone();
        self.config = config.gateway_timeout(unwrap_f64_to_duration(gateway_timeout))
    }
}
