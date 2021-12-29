use std::time::Duration;

use async_trait::async_trait;
use pyo3::exceptions::PyNotImplementedError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use songbird::events::context_data::{ConnectData, DisconnectKind, DisconnectReason};
use songbird::model::SpeakingState;
use songbird::{CoreEvent, Event, EventContext, EventHandler, TrackEvent};

use crate::track_handle::{PyTrackHandle, PyTrackState};
use crate::utils::unwrap_f64_to_duration;

#[derive(Clone)]
pub struct EventHanlder {
    pub coro: PyObject,
    pub event_loop: PyObject,
}

impl EventHanlder {
    pub fn new(coro: PyObject, event_loop: PyObject) -> Self {
        Self { coro, event_loop }
    }

    fn call_event(&self, py: Python, ctx: &EventContext) -> Result<PyObject, PyErr> {
        let asyncio = py.import("asyncio")?;
        let ensure_future = asyncio.getattr("ensure_future")?;

        let coro_wrapper: Py<PyAny> = PyModule::from_code(
            py,
            "def wrap(coro, args):
                if not isinstance(args, tuple):
                    args = (args,)
                return coro(*args)
            ",
            "",
            "",
        )?
        .getattr("wrap")?
        .into();

        let coro = coro_wrapper.call1(py, (&self.coro, event_to_py(py, ctx)?))?;

        let kwargs = PyDict::from_sequence(py, [("loop", &self.event_loop)].into_py(py))?;

        let res = ensure_future.call((coro,), Some(kwargs))?;

        Ok(PyObject::from(res))
    }
}

#[async_trait]
impl EventHandler for EventHanlder {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        Python::with_gil(|py| match self.call_event(py, ctx) {
            Ok(_) => None,
            Err(e) => {
                e.print_and_set_sys_last_vars(py);
                None
            }
        })
    }
}

fn event_to_py(py: Python, event: &EventContext) -> PyResult<PyObject> {
    match event {
        EventContext::Track(track_array) => Ok((
            PyTrackState::from(*track_array[0].0),
            PyTrackHandle::from(track_array[0].1.clone()),
        )
            .into_py(py)),
        EventContext::SpeakingStateUpdate(speaking) => Ok(PySpeaking {
            delay: speaking.delay,
            speaking: PySpeakingState::from(speaking.speaking),
            ssrc: speaking.ssrc,
            user_id: match speaking.user_id {
                Some(id) => Some(id.0),
                None => None,
            },
        }
        .into_py(py)),
        EventContext::SpeakingUpdate(data) => Ok(PySpeakingUpdateData {
            speaking: data.speaking,
            ssrc: data.ssrc,
        }
        .into_py(py)),
        EventContext::ClientConnect(data) => Ok(PyClientConnect {
            audio_ssrc: data.audio_ssrc,
            user_id: data.user_id.0,
            video_ssrc: data.video_ssrc,
        }
        .into_py(py)),
        EventContext::ClientDisconnect(disconnect) => Ok(disconnect.user_id.0.into_py(py)),
        EventContext::DriverConnect(connect) => Ok(PyConnectData::from(connect).into_py(py)),
        EventContext::DriverReconnect(connect) => Ok(PyConnectData::from(connect).into_py(py)),
        EventContext::DriverDisconnect(disconnect) => Ok(PyDisconnectData {
            kind: PyDisconnectKind::from(disconnect.kind),
            reason: match disconnect.reason {
                Some(reason) => Some(PyDisconnectReason::from(reason)),
                None => None,
            },
            channel_id: match disconnect.channel_id {
                Some(id) => Some(id.0),
                None => None,
            },
            guild_id: disconnect.guild_id.0,
            session_id: disconnect.session_id.to_string(),
        }
        .into_py(py)),
        _ => Err(PyNotImplementedError::new_err(format!(
            "{:?} is not implemented or deprecated",
            event
        ))),
    }
}

#[pyclass(name = "Event")]
#[derive(Clone)]
pub struct PyEvent {
    pub event: Event,
}

impl PyEvent {
    fn from(event: Event) -> Self {
        Self { event }
    }
}

#[allow(non_snake_case)]
#[pymethods]
impl PyEvent {
    #[staticmethod]
    #[args(phase = "None")]
    /// Repeats every duration.
    fn periodic(duration: f64, phase: Option<f64>) -> Self {
        Self::from(Event::Periodic(
            Duration::from_secs_f64(duration),
            unwrap_f64_to_duration(phase),
        ))
    }
    #[staticmethod]
    /// Event fired after the delay value.
    fn delayed(duration: f64) -> Self {
        Self::from(Event::Delayed(Duration::from_secs_f64(duration)))
    }

    #[staticmethod]
    fn Cancel() -> Self {
        Self::from(Event::Cancel)
    }

    // Track Events

    #[classattr]
    /// The attached track has resumed playing.
    /// This event will not fire when a track first starts, but will fire
    /// when a track changes from, e.g., paused to playing. This is most
    /// relevant for queue users.
    fn Play() -> Self {
        Self::from(Event::Track(TrackEvent::Play))
    }
    #[classattr]
    /// The track has been paused.
    fn Pause() -> Self {
        Self::from(Event::Track(TrackEvent::Pause))
    }
    #[classattr]
    /// The track ended.
    fn End() -> Self {
        Self::from(Event::Track(TrackEvent::End))
    }
    /// The track looped.
    #[classattr]
    fn Loop() -> Self {
        Self::from(Event::Track(TrackEvent::Loop))
    }

    // Core Events

    #[classattr]
    /// Fired on receipt of a speaking state update from another host.
    fn SpeakingStateUpdate() -> Self {
        Self::from(Event::Core(CoreEvent::SpeakingStateUpdate))
    }
    #[classattr]
    /// Fires when a source starts speaking, or stops speaking (i.e., 5 consecutive silent frames).
    fn SpeakingUpdate() -> Self {
        Self::from(Event::Core(CoreEvent::SpeakingUpdate))
    }
    #[classattr]
    /// Fires on receipt of a voice packet from another stream in the voice call.
    fn VoicePacket() -> Self {
        Self::from(Event::Core(CoreEvent::VoicePacket))
    }
    #[classattr]
    /// Fires on receipt of an RTCP packet, containing various call stats such as latency reports.
    fn RtcpPacket() -> Self {
        Self::from(Event::Core(CoreEvent::RtcpPacket))
    }
    #[classattr]
    /// Fires whenever a user connects to the same stream as the bot.
    fn ClientConnect() -> Self {
        Self::from(Event::Core(CoreEvent::ClientConnect))
    }
    #[classattr]
    /// Fires whenever a user disconnects from the same stream as the bot.
    fn ClientDisconnect() -> Self {
        Self::from(Event::Core(CoreEvent::ClientDisconnect))
    }
    #[classattr]
    /// Fires when this driver successfully connects to a voice channel.
    fn DriverConnect() -> Self {
        Self::from(Event::Core(CoreEvent::DriverConnect))
    }
    #[classattr]
    /// Fires when this driver successfully reconnects after a network error.
    fn DriverReconnect() -> Self {
        Self::from(Event::Core(CoreEvent::DriverReconnect))
    }
    /// Fires when this driver fails to connect to, or drops from, a voice channel.
    #[classattr]
    fn DriverDisconnect() -> Self {
        Self::from(Event::Core(CoreEvent::DriverDisconnect))
    }
}

#[pyclass(name = "SpeakingState")]
#[derive(Clone)]
pub struct PySpeakingState {
    pub speaking_state: SpeakingState,
}

impl PySpeakingState {
    fn from(speaking_state: SpeakingState) -> Self {
        Self { speaking_state }
    }
}

#[allow(non_snake_case)]
#[pymethods]
impl PySpeakingState {
    #[classattr]
    fn Microphone() -> Self {
        Self::from(SpeakingState::MICROPHONE)
    }
    #[classattr]
    fn Soundshare() -> Self {
        Self::from(SpeakingState::SOUNDSHARE)
    }
    #[classattr]
    fn Priority() -> Self {
        Self::from(SpeakingState::PRIORITY)
    }
}

#[pyclass(name = "Speaking")]
pub struct PySpeaking {
    #[pyo3(get)]
    delay: Option<u32>,
    #[pyo3(get)]
    speaking: PySpeakingState,
    #[pyo3(get)]
    ssrc: u32,
    #[pyo3(get)]
    user_id: Option<u64>,
}

#[pyclass(name = "SpeakingUpdateData")]
pub struct PySpeakingUpdateData {
    #[pyo3(get)]
    pub speaking: bool,
    #[pyo3(get)]
    pub ssrc: u32,
}

#[pyclass(name = "ClientConnect")]
pub struct PyClientConnect {
    #[pyo3(get)]
    audio_ssrc: u32,
    #[pyo3(get)]
    user_id: u64,
    #[pyo3(get)]
    video_ssrc: u32,
}

#[pyclass(name = "ConnectData")]
pub struct PyConnectData {
    #[pyo3(get)]
    channel_id: Option<u64>,
    #[pyo3(get)]
    guild_id: u64,
    #[pyo3(get)]
    session_id: String,
    #[pyo3(get)]
    server: String,
    #[pyo3(get)]
    ssrc: u32,
}

impl PyConnectData {
    fn from(connect: &ConnectData) -> Self {
        Self {
            channel_id: match connect.channel_id {
                Some(id) => Some(id.0),
                None => None,
            },
            guild_id: connect.guild_id.0,
            session_id: connect.session_id.to_string(),
            server: connect.server.to_string(),
            ssrc: connect.ssrc,
        }
    }
}

#[pyclass(name = "DisconnectData")]
pub struct PyDisconnectData {
    #[pyo3(get)]
    kind: PyDisconnectKind,
    #[pyo3(get)]
    reason: Option<PyDisconnectReason>,
    #[pyo3(get)]
    channel_id: Option<u64>,
    #[pyo3(get)]
    guild_id: u64,
    #[pyo3(get)]
    session_id: String,
}

#[pyclass(name = "DisconnectKind")]
#[derive(Clone)]
pub struct PyDisconnectKind {
    pub kind: DisconnectKind,
}

impl PyDisconnectKind {
    fn from(kind: DisconnectKind) -> Self {
        Self { kind }
    }
}

#[pymethods]
#[allow(non_snake_case)]
impl PyDisconnectKind {
    #[classattr]
    /// The voice driver failed to connect to the server.
    fn Connect() -> Self {
        Self::from(DisconnectKind::Connect)
    }
    #[classattr]
    /// The voice driver failed to reconnect to the server.
    fn Reconnect() -> Self {
        Self::from(DisconnectKind::Reconnect)
    }
    #[classattr]
    /// The voice connection was terminated mid-session by either the user or Discord.
    fn Runtime() -> Self {
        Self::from(DisconnectKind::Runtime)
    }
}

#[pyclass(name = "DisconnectReason")]
#[derive(Clone)]
pub struct PyDisconnectReason {
    pub reason: DisconnectReason,
}

impl PyDisconnectReason {
    fn from(reason: DisconnectReason) -> Self {
        Self { reason }
    }
}

#[allow(non_snake_case)]
#[pymethods]
impl PyDisconnectReason {
    #[classattr]
    /// This (re)connection attempt was dropped due to another request.
    fn AttemptDiscarded() -> Self {
        Self::from(DisconnectReason::AttemptDiscarded)
    }
    #[classattr]
    /// Songbird had an internal error.
    fn Internal() -> Self {
        Self::from(DisconnectReason::Internal)
    }
    #[classattr]
    /// A host-specific I/O error caused the fault; this is likely transient, and should be retried some time later.
    fn Io() -> Self {
        Self::from(DisconnectReason::Io)
    }
    #[classattr]
    /// Songbird and Discord disagreed on the protocol used to establish a voice connection.
    fn ProtocolViolation() -> Self {
        Self::from(DisconnectReason::ProtocolViolation)
    }
    #[classattr]
    /// A voice connection was not established in the specified time.
    fn TimedOut() -> Self {
        Self::from(DisconnectReason::TimedOut)
    }
    #[classattr]
    /// The Websocket connection was closed by Discord.
    fn WsClosed() -> Self {
        Self::from(DisconnectReason::WsClosed(None))
    }
}
