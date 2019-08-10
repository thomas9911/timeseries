use core::borrow::Borrow;
use core::ops::RangeBounds;
use std::collections::BTreeMap;

// use crate::traits::{BtreeMapTrait, BtreeMapViews};
use crate::traits::{BtreeMapTrait, TableTrait};
use crate::TableError;
use chrono::Datelike;
use std::collections::btree_map::{Entry, Iter, IterMut, Keys, Range, RangeMut, Values, ValuesMut};

#[derive(Debug, PartialEq)]
pub struct Table<U, V>
where
    U: Datelike + std::cmp::Ord,
{
    pub headers: Vec<String>,
    pub data: BTreeMap<U, Vec<V>>,
}

impl<U, V> Table<U, V>
where
    U: Datelike + std::cmp::Ord,
{
    pub fn new_btreemap(headers: Vec<String>, data: BTreeMap<U, Vec<V>>) -> Table<U, V> {
        Table { headers, data }
    }

    pub fn new(
        headers: Vec<String>,
        time_data: Vec<U>,
        data: Vec<Vec<V>>,
    ) -> Result<Table<U, V>, TableError> {
        let data = Self::to_btreemap(time_data, data)?;
        Ok(Table::new_btreemap(headers, data))
    }

    fn to_btreemap(
        time_data: Vec<U>,
        data: Vec<Vec<V>>,
    ) -> Result<BTreeMap<U, Vec<V>>, TableError> {
        let len = match data.get(0) {
            Some(x) => x.len(),
            None => return Err(TableError::new("data is empty")),
        };
        let mut tree_data: BTreeMap<U, Vec<V>> = BTreeMap::new();
        if data.len() != time_data.len() {
            return Err(TableError::new("time and data length should be equal"));
        }

        for (k, v) in time_data.into_iter().zip(data.into_iter()) {
            if v.len() != len {
                return Err(TableError::new("all rows should have equal length"));
            }
            tree_data.insert(k, v);
        }
        Ok(tree_data)
    }
}

impl<U, V> BtreeMapTrait<U, Vec<V>> for Table<U, V>
where
    U: Datelike + std::cmp::Ord,
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
    U: Datelike + std::cmp::Ord + Clone,
    V: Clone,
{
    fn range_object_owned<T: ?Sized, R>(&self, range: R) -> Table<U, V>
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
}

#[cfg(test)]
mod array_test {
    use crate::Table;
    use crate::{s, vec2};
    use std::collections::BTreeMap;

    macro_rules! dtu {
        ($t:expr) => {
            chrono::DateTime::parse_from_rfc3339($t).unwrap()
        };
    }

    fn new_table<'a>() -> Table<chrono::DateTime<chrono::FixedOffset>, &'a str> {
        let now: chrono::DateTime<chrono::FixedOffset> = chrono::Utc::now().into();
        let headers = vec![s!("h1"), s!("h2")];

        let times = vec![dtu!("2019-01-01T12:00:00Z"), now];
        let d = vec2![["Hark", "Bark"], ["Hans", "kaas"]];

        Table::new(headers, times, d).unwrap()
    }

    fn new_table_large<'a>() -> Table<chrono::DateTime<chrono::FixedOffset>, &'a str> {
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

        Table::new(headers, times, d).unwrap()
    }

    #[test]
    fn table_is_same_with_btreemap() {
        let now: chrono::DateTime<chrono::FixedOffset> = chrono::Utc::now().into();
        let headers = vec![s!("h1"), s!("h2")];

        let times = vec![dtu!("2019-01-01T12:00:00Z"), now.clone()];
        let d = vec2![["Hark", "Bark"], ["Hans", "kaas"]];

        let t1 = Table::new(headers.clone(), times, d).unwrap();

        let mut d = BTreeMap::new();
        d.insert(dtu!("2019-01-01T12:00:00Z"), vec!["Hark", "Bark"]);
        d.insert(now, vec!["Hans", "kaas"]);

        let t2 = Table::new_btreemap(headers, d);

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
        let t1 = new_table_large();

        let mut t2 = BTreeMap::new();
        for (k, v) in t1.range(dtu!("2019-01-03T12:00:00Z")..dtu!("2019-01-05T12:00:00Z")) {
            t2.insert(*k, v.clone());
        }

        let expected = Table::new(
            vec![s!("number"), s!("text")],
            vec![dtu!("2019-01-03T12:00:00Z"), dtu!("2019-01-04T12:00:00Z")],
            vec2![["3", "Test03"], ["4", "Test04"]],
        )
        .unwrap();

        assert_eq!(t2, expected.data);
    }

    #[test]
    fn table_btree_trait_range_object_owned() {
        use crate::TableTrait;
        let t1 = new_table_large();

        let t2 = t1.range_object_owned(dtu!("2019-01-03T12:00:00Z")..dtu!("2019-01-05T12:00:00Z"));

        let expected = Table::new(
            vec![s!("number"), s!("text")],
            vec![dtu!("2019-01-03T12:00:00Z"), dtu!("2019-01-04T12:00:00Z")],
            vec2![["3", "Test03"], ["4", "Test04"]],
        )
        .unwrap();

        assert_eq!(t2, expected);
    }
}
