use core::borrow::Borrow;
use core::ops::RangeBounds;

use std::collections::btree_map::{Entry, Iter, IterMut, Keys, Range, RangeMut, Values, ValuesMut};
use crate::enums::IndexOrColumn;
use crate::TableError;

pub trait BtreeMapTrait<K: Ord, V> {
    fn clear(&mut self);
    fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord;
    // fn get_key_value<Q: ?Sized>(&self, k: &Q) -> Option<(&K, &V)>
    //     where K: Borrow<Q>,
    //           Q: Ord;
    fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord;
    fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Ord;
    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord;
    fn append(&mut self, other: &mut Self);
    fn range<T: ?Sized, R>(&self, range: R) -> Range<'_, K, V>
    where
        T: Ord,
        K: Borrow<T>,
        R: RangeBounds<T>;
    fn range_mut<T: ?Sized, R>(&mut self, range: R) -> RangeMut<'_, K, V>
    where
        T: Ord,
        K: Borrow<T>,
        R: RangeBounds<T>;
    fn entry(&mut self, key: K) -> Entry<'_, K, V>;
    fn split_off<Q: ?Sized + Ord>(&mut self, key: &Q) -> Self
    where
        K: Borrow<Q>;
    // }
    // pub trait BtreeMapViews<K, V> {
    fn iter(&self) -> Iter<'_, K, V>;
    fn iter_mut(&mut self) -> IterMut<'_, K, V>;
    fn keys<'a>(&'a self) -> Keys<'a, K, V>;
    fn values<'a>(&'a self) -> Values<'a, K, V>;
    fn values_mut(&mut self) -> ValuesMut<'_, K, V>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

pub trait TableTrait<K: Ord, V, B: BtreeMapTrait<K, V>> {
    fn slice_owned<T: ?Sized, R>(&self, range: R) -> B
    where
        T: Ord,
        K: Borrow<T>,
        R: RangeBounds<T>;
    fn slice_inplace<T: ?Sized, R>(&mut self, range: R)
    where
        T: Ord,
        K: Borrow<T>,
        R: RangeBounds<T>;
    fn headers(&self) -> &[String];
    fn swap_columns<X, Y>(&mut self, a: X, b: Y) -> Result<(), TableError>
    where
        X: Into<IndexOrColumn>,
        Y: Into<IndexOrColumn>;
    fn swap(&mut self, a: usize, b: usize) -> Result<(), TableError>;
}
