use chrono::{DateTime, Utc, Duration};
fn power_9(base:i64) -> i64 {
    base.pow(9)
}

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + Duration::seconds(power_9(10))
}
