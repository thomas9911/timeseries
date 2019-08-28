#![cfg(all(feature = "unqlite", feature = "bincode", feature = "seahash"))]

use crate::{BtreeMapTrait, Table, TableMetaTrait};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use unqlite::{Cursor, Transaction, UnQLite, KV};

#[derive(Debug)]
pub enum LoadError {
    Bincode(std::boxed::Box<bincode::ErrorKind>),
    DbTableError(DbTableError),
    UnQLite(unqlite::Error),
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<std::boxed::Box<bincode::ErrorKind>> for LoadError {
    fn from(err: std::boxed::Box<bincode::ErrorKind>) -> LoadError {
        LoadError::Bincode(err)
    }
}

impl From<unqlite::Error> for LoadError {
    fn from(err: unqlite::Error) -> LoadError {
        LoadError::UnQLite(err)
    }
}

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

impl<U, V> Table<U, V>
where
    U: std::fmt::Debug + Clone + std::cmp::Ord + serde::de::DeserializeOwned + serde::Serialize,
    V: std::fmt::Debug + Clone + serde::de::DeserializeOwned + serde::Serialize,
{
    pub fn from_unqlite<P: AsRef<str>>(filename: P) -> Result<Table<U, V>, LoadError> {
        let db = UnQLite::create(filename);
        let mut first = db
            .first()
            .ok_or(LoadError::DbTableError(DbTableError::DbDoesNotExist))?;

        // let (header_key, header_value) = first.key_value();

        let mut headers: DbObject<Vec<String>> = DbObject::new(Vec::new());
        let mut metadata: DbObject<Option<HashMap<String, String>>> = DbObject::new(None);
        let mut btable = BTreeMap::new();
        // let mut table = Table::new_btreemap(headers, std::collections::BTreeMap::new());

        let mut n = true;
        let (item_k, item_v) = first.key_value();
        if let Ok(x) = bincode::deserialize::<String>(&item_k) {
            if x == "__META_DATA" {
                n = false;
                metadata = bincode::deserialize(&item_v)?;
            }
            if x == "__HEADER" {
                n = false;
                headers = bincode::deserialize(&item_v)?;
            }
        }

        if n {
            let k: U = bincode::deserialize(&item_k)?;
            let v: DbObject<Vec<V>> = bincode::deserialize(&item_v)?;
            // let v: Vec<V> = bincode::deserialize(&item_v)?;
            btable.insert(k, v.item);
        }

        // let mut first = db
        //     .first()
        //     .ok_or(LoadError::DbTableError(DbTableError::DbDoesNotExist))?;
        while let Some(cursor) = first.next() {
            let mut n = true;
            let (item_k, item_v) = cursor.key_value();
            if let Ok(x) = bincode::deserialize::<String>(&item_k) {
                if x == "__META_DATA" {
                    n = false;
                    metadata = bincode::deserialize(&item_v)?;
                }
                if x == "__HEADER" {
                    n = false;
                    headers = bincode::deserialize(&item_v)?;
                }
            }

            if n {
                let k: U = bincode::deserialize(&item_k)?;
                let v: DbObject<Vec<V>> = bincode::deserialize(&item_v)?;
                // let v: Vec<V> = bincode::deserialize(&item_v)?;
                btable.insert(k, v.item);
            }
            first = cursor;
        }
        if headers.item == Vec::<String>::new() {
            return Err(LoadError::DbTableError(DbTableError::DbHeaderDoesNotExist));
        }

        let mut table = Table::new_btreemap(headers.item, btable);
        if let Some(x) = metadata.item {
            table.set_meta_data(x)
        }
        Ok(table)
    }

    pub fn save_unqlite<P: AsRef<str>>(&self, filename: P) -> Result<(), LoadError> {
        let db = UnQLite::create(filename);

        match db.first() {
            Some(_) => return Err(LoadError::DbTableError(DbTableError::DbExists)),
            None => (),
        };

        db.kv_store(
            bincode::serialize("__HEADER")?,
            bincode::serialize(&DbObject::new(self.headers.clone()))?,
        )?;

        db.kv_store(
            bincode::serialize("__META_DATA")?,
            bincode::serialize(&DbObject::new(self.meta_data.clone()))?,
        )?;

        for (k, v) in self.iter() {
            let tmp = DbObject::new(v.clone());
            db.kv_store(bincode::serialize(k)?, bincode::serialize(&tmp)?)?;
        }
        Ok(())
    }

    pub fn update_unqlite<P: AsRef<str>>(&self, filename: P) -> Result<Vec<U>, LoadError> {
        let db = UnQLite::create(filename);
        let mut changed_keys = Vec::new();

        match db.first() {
            Some(_) => (),
            None => return Err(LoadError::DbTableError(DbTableError::DbDoesNotExist)),
        };

        let header_key = bincode::serialize("__HEADER")?;
        let db_table_header = DbObject::new(self.headers.clone());
        match db.kv_fetch(&header_key) {
            Ok(x) => {
                let db_object: DbObject<Vec<String>> = bincode::deserialize(&x)?;
                if db_table_header != db_object {
                    db.kv_store(header_key, bincode::serialize(&db_table_header)?)?;
                }
            }
            Err(_e) => {
                db.kv_store(header_key, bincode::serialize(&db_table_header)?)?;
            }
        }

        let meta_key = bincode::serialize("__META_DATA")?;
        let db_meta_data = DbObject::new(self.meta_data.clone());
        match db.kv_fetch(&meta_key) {
            Ok(x) => {
                let db_object: DbObject<Option<HashMap<String, String>>> =
                    bincode::deserialize(&x)?;
                if db_meta_data != db_object {
                    db.kv_store(meta_key, bincode::serialize(&db_meta_data)?)?;
                }
            }
            Err(_e) => {
                db.kv_store(meta_key, bincode::serialize(&db_meta_data)?)?;
            }
        }

        for (k, v) in self.iter() {
            let tmp = DbObject::new(v.clone());
            let key = bincode::serialize(k)?;
            match db.kv_fetch(&key) {
                Ok(x) => {
                    let db_object: DbObject<Vec<V>> = bincode::deserialize(&x)?;
                    if tmp != db_object {
                        db.kv_store(key, bincode::serialize(&tmp)?)?;
                        changed_keys.push(k.clone());
                    }
                }
                Err(_e) => {
                    db.kv_store(key, bincode::serialize(&tmp)?)?;
                    changed_keys.push(k.clone());
                }
            }
        }
        Ok(changed_keys)
    }

    pub fn save_unqlite_override<P: AsRef<str>>(&self, filename: P) -> Result<(), LoadError> {
        match self.save_unqlite(&filename) {
            Ok(x) => return Ok(x),
            Err(LoadError::DbTableError(DbTableError::DbExists)) => {
                self.delete_unqlite(&filename)?;
                self.save_unqlite(&filename)?;
            }
            Err(e) => return Err(e),
        };
        Ok(())
    }

    pub fn delete_unqlite<P: AsRef<str>>(&self, filename: P) -> Result<(), LoadError> {
        let db = UnQLite::create(filename);
        let mut first = match db.first() {
            None => return Ok(()),
            Some(x) => x,
        };
        while let Some(cursor) = first.delete() {
            first = cursor
        }

        db.commit().unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod dbobject {
    use crate::unqlite_structs::DbObject;

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
