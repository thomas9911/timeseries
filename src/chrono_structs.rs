#![cfg(feature = "chrono")]

use crate::Table;
use chrono::Datelike;

pub type DateTable<U, V> = Table<U, V>;

impl<U, V> DateTable<U, V>
where
    U: Datelike + std::cmp::Ord + Clone,
    V: Clone,
{
    pub fn cheese(){}
}

#[cfg(test)]
mod array_test {
    use crate::vec2;
    use crate::DateTable;
    use std::collections::BTreeMap;

    macro_rules! dtu {
        ($t:expr) => {
            chrono::DateTime::parse_from_rfc3339($t).unwrap()
        };
    }

    macro_rules! s {
        ($t:expr) => {
            String::from($t)
        };
    }

    fn new_table<'a>() -> DateTable<chrono::DateTime<chrono::FixedOffset>, &'a str> {
        let now: chrono::DateTime<chrono::FixedOffset> = chrono::Utc::now().into();
        let headers = vec![s!("h1"), s!("h2")];

        let times = vec![dtu!("2019-01-01T12:00:00Z"), now];
        let d = vec2![["Hark", "Bark"], ["Hans", "kaas"]];

        DateTable::new(headers, times, d).unwrap()
    }

    fn new_table_long<'a>() -> DateTable<chrono::DateTime<chrono::FixedOffset>, &'a str> {
        let headers = vec![s!("number"), s!("text")];

        let times = vec![
            dtu!("2019-01-01T12:00:00Z"),
            dtu!("2019-01-02T12:00:00Z"),
            dtu!("2019-01-03T12:00:00Z"),
            dtu!("2019-01-04T12:00:00Z"),
            dtu!("2019-01-05T12:00:00Z"),
            dtu!("2019-01-06T12:00:00Z"),
            dtu!("2019-01-07T12:00:00Z"),
            dtu!("2019-01-08T12:00:00Z"),
            dtu!("2019-01-09T12:00:00Z"),
            dtu!("2019-01-10T12:00:00Z"),
            dtu!("2019-01-11T12:00:00Z"),
            dtu!("2019-01-12T12:00:00Z"),
            dtu!("2019-01-13T12:00:00Z"),
        ];
        let d = vec2![
            ["1", "Test01"],
            ["2", "Test02"],
            ["3", "Test03"],
            ["4", "Test04"],
            ["5", "Test05"],
            ["6", "Test06"],
            ["7", "Test07"],
            ["8", "Test08"],
            ["9", "Test09"],
            ["10", "Test10"],
            ["11", "Test11"],
            ["12", "Test12"],
            ["13", "Test13"],
        ];

        DateTable::new(headers, times, d).unwrap()
    }

    fn new_table_large<'a>() -> DateTable<chrono::DateTime<chrono::FixedOffset>, &'a str> {
        let headers = vec![s!("number"), s!("text"), s!("test"), s!("data")];

        let times = vec![
            dtu!("2019-01-01T12:00:00Z"),
            dtu!("2019-01-02T12:00:00Z"),
            dtu!("2019-01-03T12:00:00Z"),
            dtu!("2019-01-04T12:00:00Z"),
            dtu!("2019-01-05T12:00:00Z"),
            dtu!("2019-01-06T12:00:00Z"),
        ];
        let d = vec2![
            ["1", "Test01", "test", "abcd"],
            ["2", "Test02", "test", "efgh"],
            ["3", "Test03", "test", "ijkl"],
            ["4", "Test04", "test", "mnop"],
            ["5", "Test05", "test", "qrst"],
            ["6", "Test06", "test", "uvwx"],
        ];

        DateTable::new(headers, times, d).unwrap()
    }

    #[test]
    fn table_is_same_with_btreemap() {
        let now: chrono::DateTime<chrono::FixedOffset> = chrono::Utc::now().into();
        let headers = vec![s!("h1"), s!("h2")];

        let times = vec![dtu!("2019-01-01T12:00:00Z"), now.clone()];
        let d = vec2![["Hark", "Bark"], ["Hans", "kaas"]];

        let t1 = DateTable::new(headers.clone(), times, d).unwrap();

        let mut d = BTreeMap::new();
        d.insert(dtu!("2019-01-01T12:00:00Z"), vec!["Hark", "Bark"]);
        d.insert(now, vec!["Hans", "kaas"]);

        let t2 = DateTable::new_btreemap(headers, d);

        assert_eq!(t1, t2);
    }

    #[test]
    fn table_btree_trait_iter() {
        use crate::BtreeMapTrait;

        let t1 = new_table();
        let date = dtu!("2019-01-01T12:00:00Z");
        let mut i1 = t1.iter();
        assert_eq!(Some((&date, &vec!["Hark", "Bark"])), i1.next());
    }

    #[test]
    fn table_btree_trait_get() {
        use crate::BtreeMapTrait;

        let t1 = new_table();
        let date = dtu!("2019-01-01T12:00:00Z");
        let i1 = t1.get(&date);
        assert_eq!(Some(&vec!["Hark", "Bark"]), i1);
    }

    #[test]
    fn table_btree_trait_get_mut() {
        use crate::BtreeMapTrait;

        let mut t1 = new_table();
        let date = dtu!("2019-01-01T12:00:00Z");
        let i1 = t1.get_mut(&date).unwrap();

        i1[0] = "Test";

        let i1 = t1.get(&date);
        assert_eq!(Some(&vec!["Test", "Bark"]), i1);
    }

    #[test]
    fn table_btree_trait_values() {
        use crate::BtreeMapTrait;

        let t1 = new_table();
        let mut i1 = t1.values();
        assert_eq!(Some(&vec!["Hark", "Bark"]), i1.next());
        assert_eq!(Some(&vec!["Hans", "kaas"]), i1.next());
    }

    #[test]
    fn table_btree_trait_range() {
        use crate::BtreeMapTrait;
        let t1 = new_table_long();

        let mut t2 = BTreeMap::new();
        for (k, v) in t1.range(dtu!("2019-01-03T12:00:00Z")..dtu!("2019-01-05T12:00:00Z")) {
            t2.insert(*k, v.clone());
        }

        let expected = DateTable::new(
            vec![s!("number"), s!("text")],
            vec![dtu!("2019-01-03T12:00:00Z"), dtu!("2019-01-04T12:00:00Z")],
            vec2![["3", "Test03"], ["4", "Test04"]],
        )
        .unwrap();

        assert_eq!(t2, expected.data);
    }

    #[test]
    fn table_btree_trait_slice_owned() {
        use crate::TableTrait;
        let t1 = new_table_long();

        let t2 = t1.slice_owned(dtu!("2019-01-03T12:00:00Z")..dtu!("2019-01-05T12:00:00Z"));

        let expected = DateTable::new(
            vec![s!("number"), s!("text")],
            vec![dtu!("2019-01-03T12:00:00Z"), dtu!("2019-01-04T12:00:00Z")],
            vec2![["3", "Test03"], ["4", "Test04"]],
        )
        .unwrap();

        assert_eq!(t2, expected);
    }

    #[test]
    fn table_btree_trait_slice_inplace() {
        use crate::TableTrait;
        let mut t1 = new_table_long();

        t1.slice_inplace(dtu!("2019-01-03T12:00:00Z")..dtu!("2019-01-05T12:00:00Z"));

        let expected = DateTable::new(
            vec![s!("number"), s!("text")],
            vec![dtu!("2019-01-03T12:00:00Z"), dtu!("2019-01-04T12:00:00Z")],
            vec2![["3", "Test03"], ["4", "Test04"]],
        )
        .unwrap();

        assert_eq!(t1, expected);
    }

    #[test]
    fn table_btree_trait_headers() {
        use crate::TableTrait;
        let t1 = new_table_long();

        let expected = [s!("number"), s!("text")];

        assert_eq!(t1.headers(), &expected);
    }

    #[test]
    fn table_btree_trait_change_column() {
        use crate::{BtreeMapTrait, TableTrait};
        let mut t1 = new_table_large();
        let t1_copy = t1.clone();

        assert_eq!(
            t1.headers(),
            &[s!("number"), s!("text"), s!("test"), s!("data")]
        );
        assert_eq!(
            t1.values().last(),
            Some(&vec!["6", "Test06", "test", "uvwx"])
        );

        t1.swap_columns("test", 3).unwrap();

        assert_eq!(
            t1.headers(),
            &[s!("number"), s!("text"), s!("data"), s!("test")]
        );
        assert_eq!(
            t1.values().last(),
            Some(&vec!["6", "Test06", "uvwx", "test"])
        );
        assert_ne!(t1, t1_copy);
    }
}
