pub mod days;

/// Run the solution for a given day.
///
/// Returns `Some(answer)` if the day is implemented, or `None` otherwise.
pub fn run_day(day: u32, part: u8, use_example: bool) -> Option<String> {
    days::run_day(day, part, use_example)
}
