use std::time::{SystemTime, Duration};

pub fn delta() -> Duration {
        let start = SystemTime::now(); 
        start.elapsed().unwrap()
}
