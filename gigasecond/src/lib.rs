use chrono::{DateTime, Utc, Duration};
fn power_9(base:i64) -> i64 {
    base.pow(9)
}

fn power( power: u32) ->  dyn Fn<i64, Output = i64>  {
    |base: i64| base.pow(power)
} 

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + Duration::seconds(power_9(10))
}
