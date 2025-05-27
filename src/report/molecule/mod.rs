mod tricolumn_colon;

pub const BPJS_LOGO: PreEscaped<&str> = PreEscaped(include_str!("./bpjs-logo.svg"));
use maud::PreEscaped;
pub use tricolumn_colon::*;

pub fn some_and_not_strip(text: Option<&str>) -> Option<&str> {
    match text {
        Some(text) if text != "-" && !text.is_empty() => Some(text),
        _ => None,
    }
}
