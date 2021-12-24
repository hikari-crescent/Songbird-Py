use std::time::Duration;

pub fn unwrap_duration(duration: Option<Duration>) -> Option<f64> {
    match duration {
        Some(duration) => Some(duration.as_secs_f64()),
        None => None,
    }
}

pub fn unwrap_f64_to_duration(duration: Option<f64>) -> Option<Duration> {
    match duration {
        Some(f) => Some(Duration::from_secs_f64(f)),
        None => None,
    }
}
