#[cfg(feature = "bincode")]
pub extern crate bincode;

#[cfg(feature = "chrono")]
pub extern crate chrono;

#[cfg(feature = "ndarray")]
pub extern crate ndarray;

#[cfg(feature = "csv")]
pub extern crate csv;

#[cfg(feature = "serde")]
pub extern crate serde;

#[cfg(feature = "num")]
pub extern crate num_traits;

#[cfg(feature = "rayon")]
pub extern crate rayon;

#[cfg(feature = "seahash")]
extern crate seahash;

#[cfg(all(test, feature = "serde_test"))]
extern crate serde_test;

#[cfg(feature = "unqlite_db")]
extern crate unqlite;

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
#[cfg(test)]
mod tests;
mod traits;
#[cfg(feature = "unqlite_db")]
mod unqlite_structs;
#[cfg(feature = "postgresql_db")]
mod postgresql_structs;
mod utils;

#[cfg(feature = "chrono")]
pub use chrono_structs::*;
pub use errors::*;
#[cfg(feature = "ndarray")]
pub use ndarray_structs::*;
pub use structs::*;
pub use traits::*;
pub use utils::*;
