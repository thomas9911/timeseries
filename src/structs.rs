use core::borrow::Borrow;
use core::ops::RangeBounds;
use std::collections::{BTreeMap, HashMap};

use crate::enums::IndexOrColumn;
use crate::traits::{BtreeMapTrait, TableMetaTrait, TableTrait};
use crate::TableError;

use std::collections::btree_map::{Entry, Iter, IterMut, Keys, Range, RangeMut, Values, ValuesMut};

///
///
///```
/// use timeseries::{vec2, Table};
///
/// let headers = vec![
///     String::from("number"),
///     String::from("text"),
///     String::from("test"),
///     String::from("data")
/// ];
///
/// let indexes = vec![1, 2, 3, 4, 5, 6];
/// let data = vec2![
///     ["1", "Test01", "test", "abcd"],
///     ["2", "Test02", "test", "efgh"],
///     ["3", "Test03", "test", "ijkl"],
///     ["4", "Test04", "test", "mnop"],
///     ["5", "Test05", "test", "qrst"],
///     ["6", "Test06", "test", "uvwx"],
/// ];
///
///  Table::new(headers, indexes, data).unwrap();
///```
///
#[derive(Debug, PartialEq, Clone)]
pub struct Table<U, V>
where
    U: std::cmp::Ord,
{
    pub headers: Vec<String>,
    pub data: BTreeMap<U, Vec<V>>,
    pub meta_data: Option<HashMap<String, String>>,
}

impl<U, V> Table<U, V>
where
    U: std::cmp::Ord,
{
    pub fn new_btreemap(headers: Vec<String>, data: BTreeMap<U, Vec<V>>) -> Table<U, V> {
        Table {
            headers,
            data,
            meta_data: None,
        }
    }

    pub fn new(
        headers: Vec<String>,
        indexes: Vec<U>,
        data: Vec<Vec<V>>,
    ) -> Result<Table<U, V>, TableError> {
        let data = Self::to_btreemap(indexes, data)?;
        Self::check_headers(&data, &headers)?;
        Ok(Table::new_btreemap(headers, data))
    }

    fn to_btreemap(indexes: Vec<U>, data: Vec<Vec<V>>) -> Result<BTreeMap<U, Vec<V>>, TableError> {
        let len = match data.get(0) {
            Some(x) => x.len(),
            None => return Err(TableError::new("data is empty")),
        };
        let mut tree_data: BTreeMap<U, Vec<V>> = BTreeMap::new();
        if data.len() != indexes.len() {
            return Err(TableError::new("time and data length should be equal"));
        }

        for (k, v) in indexes.into_iter().zip(data.into_iter()) {
            if v.len() != len {
                return Err(TableError::new("all rows should have equal length"));
            }
            tree_data.insert(k, v);
        }
        Ok(tree_data)
    }

    fn check_headers(data: &BTreeMap<U, Vec<V>>, headers: &Vec<String>) -> Result<(), TableError> {
        let len = match data.values().next() {
            Some(x) => x.len(),
            None => return Err(TableError::new("data is empty")),
        };

        if headers.len() != len {
            return Err(TableError::new("header has too many columns"));
        };
        Ok(())
    }

    pub fn iter_rows(&mut self) -> IterRows<'_, U, V, String> {
        IterRows {
            iter: self.data.iter(),
            headers: &self.headers,
        }
    }
}

#[derive(Debug)]
pub struct IterRows<'a, K, V, H> {
    pub headers: &'a [H],
    pub iter: Iter<'a, K, Vec<V>>,
}

#[derive(Debug)]
pub struct Row<'a, K, V, H> {
    pub index: &'a K,
    headers: std::slice::Iter<'a, H>,
    values: std::slice::Iter<'a, V>,
}

impl<'a, K, V, H> Iterator for IterRows<'a, K, V, H> {
    type Item = Row<'a, K, V, H>;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let (k, values) = self.iter.next()?;
        Some(Row {
            headers: self.headers.iter(),
            index: k,
            values: values.iter(),
        })
    }
}

impl<'a, K, V, H> Iterator for Row<'a, K, V, H> {
    type Item = (&'a H, &'a V);
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let h = self.headers.next()?;
        let v = self.values.next()?;
        Some((h, v))
    }
}

impl<U, V> TableMetaTrait<String, String> for Table<U, V>
where
    U: std::cmp::Ord,
{
    fn set_meta_data(&mut self, meta_data: HashMap<String, String>) {
        self.meta_data = Some(meta_data);
    }

    /// add arbitrary data to table
    fn set_meta_key(&mut self, key: String, value: String) {
        self.meta_data = match self.meta_data {
            Some(ref mut x) => {
                x.insert(key, value);
                return;
            }
            None => Some(HashMap::new()),
        };
        self.set_meta_key(key, value)
    }

    /// get arbitrary data from table
    fn get_meta_key(&mut self, key: &String) -> Option<&String> {
        match self.meta_data {
            None => None,
            Some(ref x) => x.get(key),
        }
    }
}

impl<U, V> BtreeMapTrait<U, Vec<V>> for Table<U, V>
where
    U: std::cmp::Ord,
{
    fn clear(&mut self) {
        self.data.clear()
    }
    fn get<Q: ?Sized>(&self, key: &Q) -> Option<&Vec<V>>
    where
        U: Borrow<Q>,
        Q: Ord,
    {
        self.data.get(key)
    }
    fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        U: Borrow<Q>,
        Q: Ord,
    {
        self.data.contains_key(key)
    }
    fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut Vec<V>>
    where
        U: Borrow<Q>,
        Q: Ord,
    {
        self.data.get_mut(key)
    }
    fn insert(&mut self, key: U, value: Vec<V>) -> Option<Vec<V>> {
        self.data.insert(key, value)
    }
    fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<Vec<V>>
    where
        U: Borrow<Q>,
        Q: Ord,
    {
        self.data.remove(key)
    }
    fn append(&mut self, other: &mut Self) {
        self.data.append(&mut other.data)
    }
    fn range<T: ?Sized, R>(&self, range: R) -> Range<'_, U, Vec<V>>
    where
        T: Ord,
        U: Borrow<T>,
        R: RangeBounds<T>,
    {
        self.data.range(range)
    }
    fn range_mut<T: ?Sized, R>(&mut self, range: R) -> RangeMut<'_, U, Vec<V>>
    where
        T: Ord,
        U: Borrow<T>,
        R: RangeBounds<T>,
    {
        self.data.range_mut(range)
    }
    fn entry(&mut self, key: U) -> Entry<'_, U, Vec<V>> {
        self.data.entry(key)
    }
    fn split_off<Q: ?Sized + Ord>(&mut self, key: &Q) -> Self
    where
        U: Borrow<Q>,
    {
        Self::new_btreemap(self.headers.clone(), self.data.split_off(key))
    }
    // }
    // impl<U, V> BtreeMapViews<U, Vec<V>> for Table<U, V>
    // where
    //     U: Datelike + std::cmp::Ord,
    // {
    fn iter(&self) -> Iter<'_, U, Vec<V>> {
        self.data.iter()
    }
    fn iter_mut(&mut self) -> IterMut<'_, U, Vec<V>> {
        self.data.iter_mut()
    }
    fn keys<'a>(&'a self) -> Keys<'a, U, Vec<V>> {
        self.data.keys()
    }
    fn values<'a>(&'a self) -> Values<'a, U, Vec<V>> {
        self.data.values()
    }
    fn values_mut(&mut self) -> ValuesMut<'_, U, Vec<V>> {
        self.data.values_mut()
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
impl<U, V> TableTrait<U, Vec<V>, Table<U, V>> for Table<U, V>
where
    U: std::cmp::Ord + Clone,
    V: Clone,
{
    fn slice_owned<T: ?Sized, R>(&self, range: R) -> Table<U, V>
    where
        T: Ord,
        U: Borrow<T>,
        R: RangeBounds<T>,
    {
        let mut t2: BTreeMap<U, Vec<V>> = BTreeMap::new();
        for (k, v) in self.range(range) {
            t2.insert(k.to_owned(), v.to_vec());
        }
        Table::new_btreemap(self.headers.to_owned(), t2)
    }
    fn slice_inplace<T: ?Sized, R>(&mut self, range: R)
    where
        T: Ord,
        U: Borrow<T>,
        R: RangeBounds<T>,
    {
        self.data = self
            .range(range)
            .map(|(k, v)| (k.to_owned(), v.to_vec()))
            .collect();
    }

    fn headers(&self) -> &[String] {
        self.headers.as_ref()
    }

    /// takes a string or usize and swaps the columns
    fn swap_columns<X: Into<IndexOrColumn>, Y: Into<IndexOrColumn>>(
        &mut self,
        a: X,
        b: Y,
    ) -> Result<(), TableError> {
        let enum_a = a.into();
        let enum_b = b.into();

        let index_a = match enum_a {
            IndexOrColumn::Column(owned_a) => {
                match self.headers().iter().position(|x| x == &owned_a) {
                    Some(x) => x,
                    None => return Err(TableError::new("column a not found")),
                }
            }
            IndexOrColumn::Index(x) => x,
        };

        let index_b = match enum_b {
            IndexOrColumn::Column(owned_b) => {
                match self.headers().iter().position(|x| x == &owned_b) {
                    Some(x) => x,
                    None => return Err(TableError::new("column b not found")),
                }
            }
            IndexOrColumn::Index(x) => x,
        };

        self.swap(index_a, index_b)?;
        Ok(())
    }

    fn swap(&mut self, a: usize, b: usize) -> Result<(), TableError> {
        let h_len = self.headers().len() - 1;
        if a > h_len {
            return Err(TableError::new("index number a is too high"));
        }
        if b > h_len {
            return Err(TableError::new("index number b is too high"));
        }

        for _ in self.data.values_mut().map(|x| x.swap(a, b)) {}
        self.headers.swap(a, b);
        Ok(())
    }
}

impl<U, V> Table<U, V>
where
    U: std::cmp::Ord + Clone,
    V: Clone,
{
}

#[cfg(test)]
mod array_test {
    use crate::vec2;
    use crate::Table;
    use std::collections::BTreeMap;

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
        use crate::map;
        use std::collections::HashMap;

        let mut t1 = new_table_large();

        let expected = map! {
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
        };

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
}
