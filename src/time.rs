use std::time::SystemTime;

pub fn precise_time_s() -> f64 {
    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    time.as_secs() as f64 + ((time.subsec_millis() as f64) / 1000.0)
}
