use crate::linux::common::Display;
use crate::rdev::DisplayError;

pub fn display_size(display_name: Option<&str>) -> Result<(u64, u64), DisplayError> {
    let display = Display::new(display_name).ok_or(DisplayError::NoDisplay)?;
    display.get_size().ok_or(DisplayError::NoDisplay)
}
