#![cfg(feature = "_db_base")]

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum DbTableError {
    DbExists,
    DbDoesNotExist,
    DbHeaderDoesNotExist,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DbObject<V> {
    pub item: V,
    pub hash: u64,
}

impl<V> DbObject<V>
where
    V: serde::de::DeserializeOwned + serde::Serialize,
{
    pub fn new(value: V) -> DbObject<V> {
        let mut obj = DbObject {
            item: value,
            hash: 0,
        };
        obj.rehash();
        obj
    }

    pub fn rehash(&mut self) {
        self.hash = Self::hash(&self.item);
    }

    fn hash(value: &V) -> u64 {
        seahash::hash(&bincode::serialize(value).unwrap())
    }
}

impl<V> PartialEq for DbObject<V> {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl std::fmt::Display for DbTableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod dbobject {
    use crate::DbObject;

    #[test]
    fn vec() {
        let d = DbObject::new(vec![1, 2, 3, 4]);
        assert_ne!(d.hash, 0);
    }

    #[test]
    fn string() {
        let d = DbObject::new(String::from("testing"));
        assert_ne!(d.hash, 0);
    }

    #[test]
    fn int() {
        let d = DbObject::new(15);
        assert_ne!(d.hash, 0);
    }

}
