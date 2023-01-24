use std::time::{SystemTime, Duration};

pub fn delta() -> Duration {
        let start = SystemTime::now(); let delta = start.elapsed().unwrap();
        delta
}
