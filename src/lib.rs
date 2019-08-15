#[cfg(feature = "chrono")]
pub extern crate chrono;

#[cfg(feature = "ndarray")]
pub extern crate ndarray;

#[cfg(feature = "csv")]
pub extern crate csv;

#[cfg(feature = "serde")]
pub extern crate serde;

#[cfg(all(test, feature = "serde_test"))]
extern crate serde_test;

// use chrono::DateTime;

// pub fn dt(datetime: &str) -> chrono::ParseResult<chrono::DateTime<chrono::FixedOffset>> {
//     chrono::DateTime::parse_from_rfc3339(datetime)
// }
// pub fn s(t: &str) -> String {
//     String::from(t)
// }
#[cfg(feature = "chrono")]
mod chrono_structs;
mod errors;
#[cfg(feature = "ndarray")]
mod ndarray_structs;
mod structs;
#[macro_use]
mod macros;
pub mod enums;
mod traits;
mod utils;

#[cfg(feature = "chrono")]
pub use chrono_structs::*;
pub use errors::*;
#[cfg(feature = "ndarray")]
pub use ndarray_structs::*;
pub use structs::*;
pub use traits::*;
pub use utils::*;
