#![cfg(all(feature = "unqlite", feature = "bincode"))]

extern crate tempfile;
use timeseries;

use timeseries::{vec2, BtreeMapTrait, Table, TableMetaTrait};

macro_rules! s {
    ($t:expr) => {
        String::from($t)
    };
}

fn new_table_large<'a>() -> Table<u8, String> {
    let headers = vec![s!("number"), s!("text"), s!("test"), s!("data")];

    let indexes = vec![1, 2, 3, 4, 5, 6];
    let d = vec2![
        [s!("1"), s!("Test01"), s!("test"), s!("abcd")],
        [s!("2"), s!("Test02"), s!("test"), s!("efgh")],
        [s!("3"), s!("Test03"), s!("test"), s!("ijkl")],
        [s!("4"), s!("Test04"), s!("test"), s!("mnop")],
        [s!("5"), s!("Test05"), s!("test"), s!("qrst")],
        [s!("6"), s!("Test06"), s!("test"), s!("uvwx")],
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

fn create_tmp_file<T>(test: T)
where
    T: FnOnce(&str) -> () + std::panic::UnwindSafe,
{
    let tmp_db = tempfile::NamedTempFile::new().expect("error creating test file");
    let tmp_path = tmp_db.into_temp_path();
    let tmp_path_path: &std::path::Path = tmp_path.as_ref();
    let tmp_path_str = tmp_path_path.to_str().unwrap();

    let result = std::panic::catch_unwind(|| test(tmp_path_str));

    tmp_path.close().unwrap();

    assert!(result.is_ok())
}

#[test]
fn unqlite_table_int() {
    create_tmp_file(|tmp_path_str| {
        let t = new_table_data();
        t.save_unqlite(tmp_path_str).unwrap();
        let t1: Table<u8, i32> = Table::from_unqlite(tmp_path_str).unwrap();
        t1.delete_unqlite(tmp_path_str).unwrap();
        assert_eq!(t, t1);
    })
}

#[test]
fn unqlite_table_string() {
    create_tmp_file(|tmp_path_str| {
        let t = new_table_large();
        t.save_unqlite(tmp_path_str).unwrap();
        let t1: Table<u8, String> = Table::from_unqlite(tmp_path_str).unwrap();
        t1.delete_unqlite(tmp_path_str).unwrap();
        assert_eq!(t, t1);
    })
}

#[test]
fn unqlite_table_int_metadata() {
    create_tmp_file(|tmp_path_str| {
        let mut t = new_table_data();
        t.set_meta_key(String::from("something"), String::from("a thing"));

        t.save_unqlite(tmp_path_str).unwrap();
        let t1: Table<u8, i32> = Table::from_unqlite(tmp_path_str).unwrap();

        assert_eq!(
            t1.get_meta_key(&String::from("something")),
            Some(&String::from("a thing"))
        );

        t1.delete_unqlite(tmp_path_str).unwrap();
        assert_eq!(t, t1);
    })
}

#[test]
fn unqlite_table_update_int() {
    create_tmp_file(|tmp_path_str| {
        let mut t = new_table_data();
        t.save_unqlite(tmp_path_str).unwrap();

        let t1: Table<u8, i32> = Table::from_unqlite(tmp_path_str).unwrap();

        t.insert(7, vec![70, 72, 25, 49]);
        let updated_keys = t.update_unqlite(tmp_path_str).unwrap();
        assert_eq!(vec![7], updated_keys);

        let t2: Table<u8, i32> = Table::from_unqlite(tmp_path_str).unwrap();

        assert_eq!(t, t2);

        t.insert(6, vec![60, 54, 25, 36]);
        t.insert(5, vec![50, 59, 25, 25]);
        let updated_keys = t.update_unqlite(tmp_path_str).unwrap();
        assert_eq!(vec![5, 6], updated_keys);

        let t3: Table<u8, i32> = Table::from_unqlite(tmp_path_str).unwrap();

        t3.delete_unqlite(tmp_path_str).unwrap();
        assert_eq!(t, t3);
        assert_ne!(t, t2);
        assert_ne!(t, t1);
    });
}
