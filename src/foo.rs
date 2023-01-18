use std::time::{Duration, SystemTime};

pub fn delta() -> Duration {
    let start = SystemTime::now();
    let delta = start.elapsed().unwrap();
    delta
}
