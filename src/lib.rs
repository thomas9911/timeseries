pub extern crate chrono;

#[cfg(feature = "ndarray")]
extern crate ndarray;

// use chrono::DateTime;

// pub fn dt(datetime: &str) -> chrono::ParseResult<chrono::DateTime<chrono::FixedOffset>> {
//     chrono::DateTime::parse_from_rfc3339(datetime)
// }
// pub fn s(t: &str) -> String {
//     String::from(t)
// }
mod error;
mod structs;
#[cfg(feature = "ndarray")]
mod ndarray_structs;
#[macro_use]
mod macros;

pub use error::TableError;
pub use structs::*;
#[cfg(feature = "ndarray")]
pub use ndarray_structs::*;
