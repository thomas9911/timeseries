#![cfg(all(feature = "unqlite", feature = "bincode"))]

extern crate tempfile;
use timeseries;

use timeseries::{vec2, Table};

macro_rules! s {
    ($t:expr) => {
        String::from($t)
    };
}

fn new_table_large<'a>() -> Table<u8, &'a str> {
    let headers = vec![s!("number"), s!("text"), s!("test"), s!("data")];

    let indexes = vec![1, 2, 3, 4, 5, 6];
    let d = vec2![
        ["1", "Test01", "test", "abcd"],
        ["2", "Test02", "test", "efgh"],
        ["3", "Test03", "test", "ijkl"],
        ["4", "Test04", "test", "mnop"],
        ["5", "Test05", "test", "qrst"],
        ["6", "Test06", "test", "uvwx"],
    ];

    Table::new(headers, indexes, d).unwrap()
}

fn new_table_data() -> Table<u8, i32> {
    let headers = vec![s!("p10"), s!("data"), s!("twentyfive"), s!("squares")];

    let indexes = vec![1, 2, 3, 4, 5, 6];
    let d = vec2![
        [10, 10, 25, 01],
        [20, 23, 25, 04],
        [30, 36, 25, 09],
        [40, 49, 25, 16],
        [50, 51, 25, 25],
        [60, 68, 25, 36],
    ];

    Table::new(headers, indexes, d).unwrap()
}

#[test]
fn unqlite_table() {
    let tmp_db = tempfile::NamedTempFile::new().expect("error creating test file");
    let tmp_path = tmp_db.into_temp_path();
    let tmp_path_path: &std::path::Path = tmp_path.as_ref();
    let tmp_path_str = tmp_path_path.to_str().unwrap();

    let t = new_table_data();
    t.save_unqlite(tmp_path_str).unwrap();
    let t1: Table<u8, i32> = Table::from_unqlite(tmp_path_str).unwrap();
    t1.delete_unqlite(tmp_path_str).unwrap();
    tmp_path.close();

    assert_eq!(t, t1);
}
