use time::PrimitiveDateTime as DateTime;

const GIGASECOND: time::Duration = time::Duration::seconds(1_000_000_000);

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start.saturating_add(GIGASECOND)
}
