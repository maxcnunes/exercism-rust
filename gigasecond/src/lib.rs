use time::{Duration, PrimitiveDateTime as DateTime};

// gigasecond => thousand million seconds => billion seconds => 1.000.000.000 seconds
const GIGASECOND: Duration = Duration::new(1_000_000_000, 0);

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start.checked_add(GIGASECOND).unwrap()
}
