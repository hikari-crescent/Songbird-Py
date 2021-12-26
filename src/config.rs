use std::time::Duration;

use pyo3::prelude::*;
use songbird::driver::{
    retry::{ExponentialBackoff, Retry, Strategy},
    CryptoMode, DecodeMode,
};
use songbird::Config;

use crate::utils::unwrap_f64_to_duration;

/// Variants of the XSalsa20Poly1305 encryption scheme.
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
    /// The RTP header is used as the source of nonce bytes for the packet.
    fn Normal() -> Self {
        Self::from(CryptoMode::Normal)
    }
    /// An additional random 24B suffix is used as the source of nonce bytes for the packet.
    #[classattr]
    fn Suffix() -> Self {
        Self::from(CryptoMode::Suffix)
    }
    /// An additional random 4B suffix is used as the source of nonce bytes for the packet.
    /// This nonce value increments by 1 with each packet.
    #[classattr]
    fn Lite() -> Self {
        Self::from(CryptoMode::Lite)
    }
}


// The retry strategy to use when waiting between retry attempts.
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
    /// Wait an even amount of time between each retry.
    #[staticmethod]
    #[pyo3(text_signature = "(duration: float)")]
    fn every(duration: f64) -> Self {
        Self::from(Strategy::Every(Duration::from_secs_f64(duration)))
    }

    /// Exponential backoff waiting strategy with default parameters.
    #[staticmethod]
    #[pyo3(text_signature = "()")]
    fn backoff_default() -> Self {
        Self::from(Strategy::Backoff(ExponentialBackoff::default()))
    }

    /// Exponential backoff waiting strategy.
    /// * `min` The minimum amount of time to wait between retries.
    /// * `max` Maximum amount of time to wait between retries.
    /// * `jitter` Random jitter applied to wait times. This is a percent.
    /// I.e. 0.1 will add +/-10% to generated intervals.
    #[staticmethod]
    #[pyo3(text_signature = "(min: float, max: float, jitter: float)")]
    fn backoff(min: f64, max: f64, jitter: f32) -> Self {
        Self::from(Strategy::Backoff(ExponentialBackoff {
            min: Duration::from_secs_f64(min),
            max: Duration::from_secs_f64(max),
            jitter,
        }))
    }
}

#[pyclass(name = "DecodeMode")]
/// The decode mode to use.
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
    /// Packets received from Discord are handed over to events without any changes applied.
    /// This breaks user speaking events.
    #[classattr]
    fn Pass() -> Self {
        Self::from(DecodeMode::Pass)
    }
    /// Decrypts the body of each received packet.
    #[classattr]
    fn Decrypt() -> Self {
        Self::from(DecodeMode::Decrypt)
    }
    /// Decrypts and decodes each received packet, correctly accounting for losses.
    #[classattr]
    fn Decode() -> Self {
        Self::from(DecodeMode::Decode)
    }
}

/// Configuration for the driver.
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

    #[pyo3(text_signature = "($self, crypto_mode: CryptoMode)")]
    fn set_crypto_mode(&mut self, crypto_mode: &PyCryptoMode) {
        let config = self.config.clone();
        self.config = config.crypto_mode(crypto_mode.crypto_mode)
    }

    #[pyo3(text_signature = "($self, decode_mode: DecodeMode)")]
    fn set_decode_mode(&mut self, decode_mode: &PyDecodeMode) {
        let config = self.config.clone();
        self.config = config.decode_mode(decode_mode.decode_mode)
    }

    #[pyo3(text_signature = "($self, preallocated_tracks: int)")]
    fn set_preallocated_tracks(&mut self, preallocated_tracks: usize) {
        let config = self.config.clone();
        self.config = config.preallocated_tracks(preallocated_tracks)
    }

    #[pyo3(text_signature = "($self, driver_timeout: Optional[float])")]
    fn set_driver_timeout(&mut self, driver_timeout: Option<f64>) {
        let config = self.config.clone();
        self.config = config.driver_timeout(unwrap_f64_to_duration(driver_timeout))
    }

    #[pyo3(text_signature = "($self, driver_retry: Optional[int])")]
    fn set_driver_retry(&mut self, strategy: &PyStrategy, retry_limit: Option<usize>) {
        let config = self.config.clone();
        self.config = config.driver_retry(Retry {
            strategy: strategy.strategy,
            retry_limit,
        })
    }

    #[pyo3(text_signature = "($self, crypto_mode: Optional[float])")]
    fn set_gateway_timeout(&mut self, gateway_timeout: Option<f64>) {
        let config = self.config.clone();
        self.config = config.gateway_timeout(unwrap_f64_to_duration(gateway_timeout))
    }
}
