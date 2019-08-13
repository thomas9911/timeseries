#[cfg(feature = "chrono")]
pub extern crate chrono;

#[cfg(feature = "ndarray")]
pub extern crate ndarray;

#[cfg(feature = "csv")]
pub extern crate csv;

// use chrono::DateTime;

// pub fn dt(datetime: &str) -> chrono::ParseResult<chrono::DateTime<chrono::FixedOffset>> {
//     chrono::DateTime::parse_from_rfc3339(datetime)
// }
// pub fn s(t: &str) -> String {
//     String::from(t)
// }
mod errors;
mod structs;
#[cfg(feature = "ndarray")]
mod ndarray_structs;
#[cfg(feature = "chrono")]
mod chrono_structs;
#[macro_use]
mod macros;
mod traits;
mod utils;
pub mod enums;

pub use errors::*;
pub use traits::*;
pub use structs::*;
#[cfg(feature = "chrono")]
pub use chrono_structs::*;
#[cfg(feature = "ndarray")]
pub use ndarray_structs::*;
pub use utils::*;