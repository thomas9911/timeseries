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

#[cfg(feature = "postgres")]
pub extern crate postgres;

#[cfg(feature = "rayon")]
pub extern crate rayon;

#[cfg(feature = "seahash")]
pub extern crate seahash;

#[cfg(all(test, feature = "serde_test"))]
extern crate serde_test;

#[cfg(feature = "unqlite_db")]
pub extern crate unqlite;

#[cfg(feature = "sqlite_db")]
pub extern crate rusqlite;

#[cfg(feature = "redis_db")]
pub extern crate redis;

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
#[cfg(feature = "serde")]
mod db_structs;
pub mod enums;
#[cfg(feature = "postgresql_db")]
mod postgresql_structs;
#[cfg(feature = "redis_db")]
mod redis_structs;
#[cfg(feature = "sqlite_db")]
mod sqlite_structs;
#[cfg(test)]
mod tests;
mod traits;
#[cfg(feature = "unqlite_db")]
mod unqlite_structs;
mod utils;

#[cfg(feature = "chrono")]
pub use chrono_structs::*;
#[cfg(feature = "serde")]
pub use db_structs::*;
pub use errors::*;
#[cfg(feature = "ndarray")]
pub use ndarray_structs::*;
#[cfg(feature = "postgresql_db")]
pub use postgresql_structs::*;
#[cfg(feature = "redis_db")]
pub use redis_structs::*;
#[cfg(feature = "sqlite_db")]
pub use sqlite_structs::*;
pub use structs::*;
pub use traits::*;
pub use utils::*;
