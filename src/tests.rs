use crate::vec2;
use crate::Table;
use std::collections::HashMap;

macro_rules! s {
    ($t:expr) => {
        String::from($t)
    };
}

fn new_table<'a>() -> Table<u8, &'a str> {
    let headers = vec![s!("h1"), s!("h2")];

    let indexes = vec![1, 2];
    let d = vec2![["Hark", "Bark"], ["Hans", "kaas"]];

    Table::new(headers, indexes, d).unwrap()
}

fn new_table_long<'a>() -> Table<u8, &'a str> {
    let headers = vec![s!("number"), s!("text")];

    let indexes = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
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

    Table::new(headers, indexes, d).unwrap()
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

fn new_table_data<'a>() -> Table<u8, i32> {
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

fn new_table_float_data<'a>() -> Table<u8, f32> {
    let headers = vec![s!("p10"), s!("data"), s!("twentyfive"), s!("squares")];

    let indexes = vec![1, 2, 3, 4, 5, 6];
    let d = vec2![
        [10.0, 10.0, 25.0, 01.0],
        [20.0, 23.0, 25.0, 04.0],
        [30.0, 36.0, 25.0, 09.0],
        [40.0, 49.0, 25.0, 16.0],
        [50.0, 51.0, 25.0, 25.0],
        [60.0, 68.0, 25.0, 36.0],
    ];

    Table::new(headers, indexes, d).unwrap()
}

fn map_table_large<'a>() -> HashMap<u8, HashMap<String, &'a str>> {
    map! {
        1 => map!{
            s!("number") => "1",
            s!("text") => "Test01",
            s!("test") => "test",
            s!("data") => "abcd"
        },
        2 => map!{
            s!("number") => "2",
            s!("text") => "Test02",
            s!("test") => "test",
            s!("data") => "efgh"
        },
        3 => map!{
            s!("number") => "3",
            s!("text") => "Test03",
            s!("test") => "test",
            s!("data") => "ijkl"
        },
        4 => map!{
            s!("number") => "4",
            s!("text") => "Test04",
            s!("test") => "test",
            s!("data") => "mnop"
        },
        5 => map!{
            s!("number") => "5",
            s!("text") => "Test05",
            s!("test") => "test",
            s!("data") => "qrst"
        },
        6 => map!{
            s!("number") => "6",
            s!("text") => "Test06",
            s!("test") => "test",
            s!("data") => "uvwx"
        }
    }
}

#[cfg(test)]
mod array_test {
    use super::{map_table_large, new_table, new_table_large, new_table_long};
    use crate::Table;
    use std::collections::BTreeMap;

    macro_rules! s {
        ($t:expr) => {
            String::from($t)
        };
    }

    #[test]
    fn table_is_same_with_btreemap() {
        let headers = vec![s!("h1"), s!("h2")];

        let indexes = vec![1, 2];
        let d = vec2![["Hark", "Bark"], ["Hans", "kaas"]];

        let t1 = Table::new(headers.clone(), indexes, d).unwrap();

        let mut d = BTreeMap::new();
        d.insert(1, vec!["Hark", "Bark"]);
        d.insert(2, vec!["Hans", "kaas"]);

        let t2 = Table::new_btreemap(headers, d);

        assert_eq!(t1, t2);
    }

    #[test]
    fn table_btree_trait_iter() {
        use crate::BtreeMapTrait;

        let t1 = new_table();
        let date = 1;
        let mut i1 = t1.iter();
        assert_eq!(Some((&date, &vec!["Hark", "Bark"])), i1.next());
    }

    #[test]
    fn table_btree_trait_get() {
        use crate::BtreeMapTrait;

        let t1 = new_table();
        let date = 1;
        let i1 = t1.get(&date);
        assert_eq!(Some(&vec!["Hark", "Bark"]), i1);
    }

    #[test]
    fn table_btree_trait_get_mut() {
        use crate::BtreeMapTrait;

        let mut t1 = new_table();
        let date = 1;
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
        for (k, v) in t1.range(3..5) {
            t2.insert(*k, v.clone());
        }

        let expected = Table::new(
            vec![s!("number"), s!("text")],
            vec![3, 4],
            vec2![["3", "Test03"], ["4", "Test04"]],
        )
        .unwrap();

        assert_eq!(t2, expected.data);
    }

    #[test]
    fn table_btree_trait_slice_owned() {
        use crate::TableTrait;
        let t1 = new_table_long();

        let t2 = t1.slice_owned(3..5);

        let expected = Table::new(
            vec![s!("number"), s!("text")],
            vec![3, 4],
            vec2![["3", "Test03"], ["4", "Test04"]],
        )
        .unwrap();

        assert_eq!(t2, expected);
    }

    #[test]
    fn table_btree_trait_slice_inplace() {
        use crate::TableTrait;
        let mut t1 = new_table_long();

        t1.slice_inplace(3..5);

        let expected = Table::new(
            vec![s!("number"), s!("text")],
            vec![3, 4],
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

    #[test]
    fn meta_data() {
        use crate::TableMetaTrait;

        let mut t1 = new_table_large();
        assert_eq!(None, t1.get_meta_key(&s!("total")));

        t1.set_meta_key(s!("total"), s!("1500"));
        t1.set_meta_key(s!("size"), s!("123kb"));

        assert_eq!(Some(&s!("1500")), t1.get_meta_key(&s!("total")));
        assert_eq!(Some(&s!("123kb")), t1.get_meta_key(&s!("size")));
        assert_eq!(None, t1.get_meta_key(&s!("some_other_thing")));
    }

    #[test]
    fn table_rows() {
        use std::collections::HashMap;

        let t1 = new_table_large();
        let expected = map_table_large();

        let mut t = HashMap::new();
        for row in t1.iter_rows() {
            let mut t1 = HashMap::new();
            let i = row.index.to_owned();

            for (k, v) in row {
                t1.insert(k.to_owned(), v.to_owned());
            }
            t.insert(i, t1);
        }
        assert_eq!(t, expected);
    }

    #[test]
    fn table_columns() {
        let t1 = new_table_large();
        let expected = vec![
            "number", "1", "2", "3", "4", "5", "6", "text", "Test01", "Test02", "Test03", "Test04",
            "Test05", "Test06", "test", "test", "test", "test", "test", "test", "test", "data",
            "abcd", "efgh", "ijkl", "mnop", "qrst", "uvwx",
        ];
        let mut t = vec![];
        for column in t1.iter_columns() {
            let h = column.header.clone();
            t.push(h);
            for (_index, val) in column {
                t.push(String::from(*val));
            }
        }
        assert_eq!(expected, t);
    }

    #[test]
    fn from_map() {
        use crate::traits::TableTrait;

        let input = map_table_large();

        let mut expected = new_table_large();

        // sort table columns
        expected.swap_columns(0, 3).unwrap();
        expected.swap_columns(1, 3).unwrap();

        let t = Table::from_map(&input);

        assert_eq!(expected, t);
    }

    #[test]
    fn from_map_checked_ok() {
        let input = map_table_large();
        assert!(Table::from_map_checked(&input).is_ok());
    }

    #[test]
    fn from_map_checked_err() {
        let mut input = map_table_large();

        input.get_mut(&3).unwrap().remove(&s!("text"));

        assert!(Table::from_map_checked(&input).is_err());
    }

    #[test]
    fn from_map_safe() {
        use crate::traits::BtreeMapTrait;
        let mut input = map_table_large();

        // remove from 3rd row the last column
        input.get_mut(&3).unwrap().remove(&s!("text"));

        let t: Table<u8, Option<&str>> = Table::from_map_safe(&input);

        let third_row = t.get(&3).unwrap();

        // the second column ("number") of the 3rd row is something
        assert_eq!(Some("3"), third_row[1]);

        // the last column 3rd row is empty
        assert_eq!(None, third_row[3]);
    }
}

#[cfg(test)]
mod folding_test {
    use super::{new_table_data, new_table_float_data, new_table_large};

    macro_rules! assert_float_eq {
        ($x:expr, $y:expr) => {
            let a: Vec<(_, u32)> = $x.iter().map(|(k, v)| (k, (v*1000.0).round() as u32)).collect();
            let b: Vec<(_, u32)> = $y.iter().map(|(k, v)| (k, (v*1000.0).round() as u32)).collect();
            assert_eq!(a, b);
        };
    }

    #[test]
    fn assert_float_eq_test_equal(){
        let t: Vec<(String, f32)> = vec![
            (s!("test"), 12.01501),
            (s!("other"), 11.1843),
        ];
        let u: Vec<(String, f32)> = vec![
            (s!("test"), 12.01498),
            (s!("other"), 11.1843),
        ];
        assert_float_eq!(t, u);
    }

    #[test]
    #[should_panic]
    fn assert_float_eq_test_not_equal(){
        let t: Vec<(String, f32)> = vec![
            (s!("test"), 12.01501),
            (s!("other"), 11.1843),
        ];
        let u: Vec<(String, f32)> = vec![
            (s!("test"), 12.01401),
            (s!("other"), 11.1843),
        ];
        assert_float_eq!(t, u);
    }



    #[test]
    fn table_fold_column_str() {
        let expected = vec![
            (s!("number"), s!("123456")),
            (s!("text"), s!("Test01Test02Test03Test04Test05Test06")),
            (s!("test"), s!("testtesttesttesttesttest")),
            (s!("data"), s!("abcdefghijklmnopqrstuvwx")),
        ];

        let t1 = new_table_large();
        let output = t1.fold_columns(String::new(), |mut x, (_k, v)| {
            x.push_str(v);
            x
        });

        assert_eq!(expected, output);
    }

    #[test]
    fn table_fold_column_number() {
        let expected = vec![
            (s!("p10"), 210),
            (s!("data"), 237),
            (s!("twentyfive"), 150),
            (s!("squares"), 91),
        ];

        let t1 = new_table_data();
        let output = t1.fold_columns(0, |x, (_k, v)| x + v);

        assert_eq!(expected, output);
    }

    #[test]
    fn table_sum_column_number() {
        let expected = vec![
            (s!("p10"), 210),
            (s!("data"), 237),
            (s!("twentyfive"), 150),
            (s!("squares"), 91),
        ];

        let t1 = new_table_data();
        let output = t1.sum_columns();

        assert_eq!(expected, output);
    }

    #[test]
    #[cfg(feature = "num")]
    fn table_avg_column_number() {
        let expected = vec![
            (s!("p10"), 35),
            (s!("data"), 39),
            (s!("twentyfive"), 25),
            (s!("squares"), 15),
        ];

        let t1 = new_table_data();
        let output = t1.avg_columns();

        assert_eq!(expected, output);
    }

    #[test]
    #[cfg(feature = "num")]
    fn table_std_column_number() {
        let expected = vec![
            (s!("p10"), 291),
            (s!("data"), 365),
            (s!("twentyfive"), 0),
            (s!("squares"), 149),
        ];

        let t1 = new_table_data();
        let output = t1.var_columns();

        assert_eq!(expected, output);
    }

    #[test]
    #[cfg(feature = "num")]
    fn table_var_column_float() {
        let expected = vec![
            (s!("p10"), 291.66666),
            (s!("data"), 364.91666),
            (s!("twentyfive"), 0.0),
            (s!("squares"), 149.13887),
        ];

        let t1 = new_table_float_data();
        let output = t1.var_columns();

        assert_eq!(expected, output);
    }

    #[test]
    #[cfg(feature = "rayon")]
    fn table_p_fold_column_number_add() {
        let expected = vec![
            (s!("p10"), 210),
            (s!("data"), 237),
            (s!("twentyfive"), 150),
            (s!("squares"), 91),
        ];

        let t1 = new_table_data();
        let output = t1.p_fold_columns(0, |x, (_k, v)| x + v, |a, b| a + b);

        assert_eq!(expected, output);
    }

    #[test]
    #[cfg(feature = "rayon")]
    fn table_p_fold_column_float_add() {
        let expected: Vec<(String, f32)> = vec![
            (s!("p10"), 210.0),
            (s!("data"), 237.0),
            (s!("twentyfive"), 150.0),
            (s!("squares"), 91.0),
        ];

        let t1 = new_table_float_data();
        let output = t1.p_fold_columns(0.0, |x, (_k, v)| x + v, |a, b| a + b);

        assert_float_eq!(expected, output);
    }

    #[test]
    #[cfg(feature = "rayon")]
    fn table_p_fold_column_number_multiply() {
        let expected = vec![
            (s!("p10"), 720000000),
            (s!("data"), 1407036960),
            (s!("twentyfive"), 244140625),
            (s!("squares"), 518400),
        ];

        let t1 = new_table_data();
        let output = t1.p_fold_columns(1, |x, (_k, v)| x * v, |a, b| a * b);

        assert_eq!(expected, output);
    }

    #[test]
    #[cfg(feature = "rayon")]
    fn table_p_fold_column_float_sum_sqrt() {
        let expected: Vec<(String, f32)> = vec![
            (s!("p10"), 34.25323),
            (s!("data"), 36.34575),
            (s!("twentyfive"), 30.0),
            (s!("squares"), 21.0),
        ];

        let t1 = new_table_float_data();
        let output = t1.p_fold_columns(0.0, |x, (_k, v)| x + v.sqrt(), |a, b| a + b);

        assert_float_eq!(expected, output);
    }

}

#[cfg(all(test, feature = "serialize"))]
mod serde_testing {
    use crate::vec2;
    use crate::Table;
    use serde_test::{assert_tokens, Token};

    fn s(x: &str) -> String {
        String::from(x)
    }

    fn new_table_large() -> Table<u8, String> {
        let headers = vec![s("number"), s("text"), s("test"), s("data")];

        let indexes = vec![1, 2, 3, 4, 5, 6];
        let d = vec2![
            [s("1"), s("Test01"), s("test"), s("abcd")],
            [s("2"), s("Test02"), s("test"), s("efgh")],
            [s("3"), s("Test03"), s("test"), s("ijkl")],
            [s("4"), s("Test04"), s("test"), s("mnop")],
            [s("5"), s("Test05"), s("test"), s("qrst")],
            [s("6"), s("Test06"), s("test"), s("uvwx")],
        ];

        Table::new(headers, indexes, d).unwrap()
    }

    fn serde_tokens() -> &'static [Token] {
        &[
            Token::Struct {
                len: 3,
                name: "Table",
            },
            Token::String("headers"),
            Token::Seq { len: Some(4) },
            Token::String("number"),
            Token::String("text"),
            Token::String("test"),
            Token::String("data"),
            Token::SeqEnd,
            Token::String("data"),
            Token::Map { len: Some(6) },
            Token::U8(1),
            Token::Seq { len: Some(4) },
            Token::String("1"),
            Token::String("Test01"),
            Token::String("test"),
            Token::String("abcd"),
            Token::SeqEnd,
            Token::U8(2),
            Token::Seq { len: Some(4) },
            Token::String("2"),
            Token::String("Test02"),
            Token::String("test"),
            Token::String("efgh"),
            Token::SeqEnd,
            Token::U8(3),
            Token::Seq { len: Some(4) },
            Token::String("3"),
            Token::String("Test03"),
            Token::String("test"),
            Token::String("ijkl"),
            Token::SeqEnd,
            Token::U8(4),
            Token::Seq { len: Some(4) },
            Token::String("4"),
            Token::String("Test04"),
            Token::String("test"),
            Token::String("mnop"),
            Token::SeqEnd,
            Token::U8(5),
            Token::Seq { len: Some(4) },
            Token::String("5"),
            Token::String("Test05"),
            Token::String("test"),
            Token::String("qrst"),
            Token::SeqEnd,
            Token::U8(6),
            Token::Seq { len: Some(4) },
            Token::String("6"),
            Token::String("Test06"),
            Token::String("test"),
            Token::String("uvwx"),
            Token::SeqEnd,
            Token::MapEnd,
            Token::String("meta_data"),
            Token::None,
            Token::StructEnd,
        ]
    }

    #[test]
    fn table_serde_de_ser() {
        let t = new_table_large();
        let expected = serde_tokens();

        assert_tokens(&t, &expected);
    }
}
