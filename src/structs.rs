use core::borrow::Borrow;
use core::ops::RangeBounds;
use std::collections::{BTreeMap, BTreeSet, HashMap};

use crate::enums::IndexOrColumn;
use crate::traits::{BtreeMapTrait, TableMetaTrait, TableTrait};
use crate::TableError;

use std::collections::btree_map::{Entry, Iter, IterMut, Keys, Range, RangeMut, Values, ValuesMut};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct Table<U, V>
where
    // U: std::cmp::Ord,
    U: std::fmt::Debug + std::cmp::Ord,
    V: std::fmt::Debug,
{
    pub headers: Vec<String>,
    pub data: BTreeMap<U, Vec<V>>,
    pub meta_data: Option<HashMap<String, String>>,
}

impl<U, V> Table<U, V>
where
    // U: std::cmp::Ord,
    // U: std::fmt::Debug + std::cmp::Ord,
    // V: std::fmt::Debug,
    U: std::fmt::Debug + std::cmp::Ord,
    V: std::fmt::Debug,
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

    ///Creates table from data in this form:
    ///```json
    /// {
    ///   1: {
    ///        "test": "testing",
    ///        "number": "1"
    ///      },
    ///   2: {
    ///        "test": "testing",
    ///        "number": "2"
    ///      },
    ///   3: {
    ///        "test": "testing",
    ///        "number": "3"
    ///      }
    /// }
    ///```
    pub fn from_map(map: &HashMap<U, HashMap<String, V>>) -> Table<U, V>
    where
        U: std::hash::Hash + Clone,
        V: Clone,
    {
        let headers = Self::map_create_headers(map);

        let mut data = BTreeMap::new();

        for (index, m) in map.iter() {
            let mut row = vec![];
            for k in headers.iter() {
                row.push(m[k].to_owned());
            }
            data.insert(index.to_owned(), row);
        }

        Table::new_btreemap(headers, data)
    }

    ///Creates table from data in this form and returns an error if it does not fit:
    ///```json
    /// {
    ///   1: {
    ///        "test": "testing",
    ///        "number": "1"
    ///      },
    ///   2: {
    ///        "test": "testing",
    ///        "number": "2"
    ///      },
    ///   3: {
    ///        "test": "testing",
    ///        "number": "3"
    ///      }
    /// }
    ///```
    pub fn from_map_checked(map: &HashMap<U, HashMap<String, V>>) -> Result<Table<U, V>, TableError>
    where
        U: std::hash::Hash + Clone,
        V: Clone,
    {
        let headers = Self::map_create_headers(map);

        let mut data = BTreeMap::new();

        for (index, m) in map.iter() {
            let mut row = vec![];
            for k in headers.iter() {
                let v = match m.get(k) {
                    Some(x) => x,
                    None => {
                        return Err(TableError::new(
                            "Objects in the map do not contain the same keys",
                        ))
                    }
                };
                row.push(v.to_owned());
            }
            data.insert(index.to_owned(), row);
        }

        Ok(Table::new_btreemap(headers, data))
    }

    ///Creates table from data in this form, and return a table where every cell is wrapped in an option object:
    ///```json
    /// {
    ///   1: {
    ///        "test": "testing",
    ///        "number": "1"
    ///      },
    ///   2: {
    ///        "test": "testing",
    ///        "number": "2",
    ///        "extra_column": "info"
    ///      },
    ///   3: {
    ///        "test": "testing",
    ///        "number": "3"
    ///      }
    /// }
    ///```
    pub fn from_map_safe(map: &HashMap<U, HashMap<String, V>>) -> Table<U, Option<V>>
    where
        U: std::hash::Hash + Clone,
        V: Clone,
    {
        let headers = Self::map_create_headers(map);

        let mut data = BTreeMap::new();

        for (index, m) in map.iter() {
            let mut row = vec![];
            for k in headers.iter() {
                row.push(m.get(k).cloned());
            }
            data.insert(index.to_owned(), row);
        }

        Table::new_btreemap(headers, data)
    }

    fn map_create_headers<X>(map: &HashMap<U, HashMap<String, X>>) -> Vec<String> {
        let mut header_set = BTreeSet::new();

        for m in map.values() {
            for k in m.keys() {
                header_set.insert(k.to_owned());
            }
        }

        header_set.into_iter().collect()
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

    // pub fn fold<B, F>(self, init: B, f: F) -> Vec<B> where
    // Self: Sized,
    // B: Clone,
    // F: FnMut(B, (&U, &V)) -> B + Copy,
    // {
    //     let mut outputs = Vec::new();
    //     for col in self.iter_columns(){
    //         let start = init.clone();
    //         outputs.push(col.fold(start, f))
    //     };
    //     outputs
    // }

    pub fn fold_columns<B, F>(&self, init: B, f: F) -> Vec<(String, B)>
    where
        Self: Sized,
        B: Clone,
        F: FnMut(B, (&U, &V)) -> B + Copy,
    {
        let mut outputs = Vec::new();
        for col in self.iter_columns() {
            let start = init.clone();
            outputs.push((col.header.to_owned(), col.fold(start, f)))
        }
        outputs
    }

    pub fn sum_columns(&self) -> Vec<(String, V)>
    where
        V: std::ops::Add + Clone + Default + From<<V as std::ops::Add>::Output>,
    {
        self.fold_columns(V::default(), |x, (_k, v)| V::from(x + v.clone()))
    }

    #[cfg(feature = "num")]
    pub fn avg_columns(&self) -> Vec<(String, V)>
    where
        V: num_traits::Num
            + num_traits::NumCast
            + Clone
            + Default
            + From<<V as std::ops::Add>::Output>,
    {
        let len: V = num_traits::cast(self.len()).unwrap();
        self.sum_columns()
            .iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned() / len.clone()))
            .collect()
    }

    #[cfg(feature = "num")]
    pub fn var_columns(&self) -> Vec<(String, V)>
    where
        V: num_traits::Num
            + num_traits::NumCast
            + std::cmp::PartialOrd
            + Clone
            + Default
            + From<<V as std::ops::Add>::Output>,
    {
        let mut t = Vec::new();
        let len: V = num_traits::cast(self.len()).unwrap();
        for (p, q) in self.iter_columns().zip(self.avg_columns()) {
            let avg = q.1;
            let mut result = V::zero();
            let header = p.header.to_owned();

            for (_, v) in p {
                let s;
                if v > &avg {
                    s = v.to_owned() - avg.to_owned();
                } else {
                    s = v.to_owned() - avg.to_owned();
                }
                result = result + s.clone() * s;
            }
            t.push((header, result.to_owned() / len.clone()))
        }
        t
    }

    pub fn iter_rows(&self) -> IterRows<'_, U, V, String> {
        IterRows {
            iter: self.data.iter(),
            headers: &self.headers,
        }
    }

    pub fn iter_columns(&self) -> IterColumns<'_, U, V, String> {
        IterColumns {
            indexes: self.data.keys(),
            values: self.data.values(),
            headers: self.headers.iter(),
            counter: (0..self.headers.len()),
        }
    }
}

impl<U, V> Table<U, V>
where
    U: std::fmt::Debug + std::cmp::Ord + Send + Sync,
    V: std::fmt::Debug + Send + Sync,
{
    /// fold columns in parallel
    /// init is the 'empty' base value to use
    /// f is the function to convert a row into the value you want
    /// g is the function that combines the values into a single value
    #[cfg(feature = "rayon")]
    pub fn p_fold_columns<B, F, G>(&self, init: B, f: F, g: G) -> Vec<(String, B)>
    where
        Self: Sized,
        B: Clone + Send + Sync,
        F: Fn(B, (&U, &V)) -> B + Copy + Send + Sync,
        G: Fn(B, B) -> B + Copy + Sync + Send,
    {
        let mut outputs = Vec::new();
        for col in self.iter_columns() {
            outputs.push((
                col.header.to_owned(),
                col.par_bridge()
                    .fold(|| init.clone(), f)
                    .reduce(|| init.clone(), g),
            ))
        }
        outputs
    }
}

// impl<'a, 'b, K: std::cmp::Ord, V>
// TableIterator<'a, IterRows<'a, K, V, String, Iter<'_, K, Vec<V>>>, IterColumns<'a, K, V, String>> for Table<K, V>{
//     fn iter_rows(&mut self) -> IterRows<'_, K, V, String, Iter<'_, K, Vec<V>>> {
//         IterRows {
//             iter: self.data.iter(),
//             headers: &self.headers[..],
//         }
//     }

//     fn iter_columns(&mut self) -> IterColumns<'a, K, V, String> {
//         IterColumns {
//             indexes: self.data.keys(),
//             values: self.data.values(),
//             headers: self.headers.iter(),
//             counter: (0..self.headers.len()),
//         }
//     }
// }

// impl<'a, U: std::cmp::Ord, V> TableIterator for Table<U, V>{
//     type RowIter<'a> = IterRows<'a, U, V, String>;
//     type ColumnIter<'a> = IterColumns<'a, U, V, String>;

//     fn iter_rows(&'a mut self) -> Self::RowIter {
//         IterRows {
//             iter: self.data.iter(),
//             headers: &self.headers,
//         }
//     }

//     fn iter_columns(&'a mut self) -> Self::ColumnIter {
//         IterColumns {
//             indexes: self.data.keys(),
//             values: self.data.values(),
//             headers: self.headers.iter(),
//             counter: (0..self.headers.len()),
//         }
//     }
// }

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

impl<'a, 'b, K, V, H> Iterator for IterRows<'a, K, V, H> {
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

#[derive(Debug)]
pub struct IterColumns<'a, K, V, H> {
    pub headers: std::slice::Iter<'a, H>,
    counter: std::ops::Range<usize>,
    indexes: Keys<'a, K, Vec<V>>,
    values: Values<'a, K, Vec<V>>,
}

#[derive(Debug)]
pub struct Column<'a, K, V, H> {
    pub header: &'a H,
    pub column_index: usize,
    indexes: Keys<'a, K, Vec<V>>,
    values: Values<'a, K, Vec<V>>,
}

impl<'a, K, V, H> Iterator for IterColumns<'a, K, V, H> {
    type Item = Column<'a, K, V, H>;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let c = self.counter.next()?;
        Some(Column {
            header: self.headers.next()?,
            column_index: c,
            indexes: self.indexes.clone(),
            values: self.values.clone(),
        })
    }
}

impl<'a, K, V, H> Iterator for Column<'a, K, V, H> {
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let k = self.indexes.next()?;
        let v = self.values.next()?.get(self.column_index)?;
        Some((k, v))
    }
}

// #[cfg(feature = "rayon")]
// impl<'a, K: Send + Sync, V: Send + Sync, H: Send + Sync> ParallelIterator for Column<'a, K, V, H> {
//     type Item = (&'a K, &'a V);

//     fn drive_unindexed<C>(self, consumer: C) -> C::Result
//     where
//         C: UnindexedConsumer<Self::Item>,
//     {
//         use rayon::iter::plumbing::*;
//         bridge(self, consumer)
//     }

//     fn opt_len(&self) -> Option<usize> {
//         Some(self.indexes.len())
//     }
// }

// #[cfg(feature = "rayon")]
// impl<'a, K, V, H> rayon::iter::ParallelBridge for Column<'a, K, V, H>
// where
//     <Column<'a, K, V, H> as Iterator>::Item: Send,
// {
//     fn par_bridge(self) -> rayon::iter::IterBridge<Self> {
//         rayon::iter::IterBridge { iter: self }
//     }
// }

// #[cfg(feature = "rayon")]
// impl<'a, K, V, H> ParallelBridge for Column<'a, K, V, H>
// where <Column<'a, K, V, H> as Iterator>::Item: Send {}

// #[cfg(feature = "rayon")]
// impl<U, V> ParallelBridge for Column<'_, U, V, std::string::String>{}

impl<U, V> TableMetaTrait<String, String> for Table<U, V>
where
    // U: std::cmp::Ord,
    U: std::fmt::Debug + std::cmp::Ord,
    V: std::fmt::Debug,
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
    // U: std::cmp::Ord,
    U: std::fmt::Debug + std::cmp::Ord,
    V: std::fmt::Debug,
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
    // U: std::cmp::Ord + Clone,
    // V: Clone,
    U: std::fmt::Debug + std::cmp::Ord + Clone,
    V: std::fmt::Debug + Clone,
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
