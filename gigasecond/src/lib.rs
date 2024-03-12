use time::ext::NumericalDuration;
use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    // gigasecond => thousand million seconds => billion seconds => 1.000.000.000 seconds
    start + 1_000_000_000.seconds()
}
