#![cfg(all(feature = "unqlite", feature = "bincode"))]

use crate::{BtreeMapTrait, Table, TableMetaTrait, TableTrait};
use std::collections::{BTreeMap, HashMap};
use unqlite::{Cursor, UnQLite, KV, Transaction};

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

impl std::fmt::Display for DbTableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<U, V> Table<U, V>
where
    U: std::fmt::Debug + std::cmp::Ord + serde::de::DeserializeOwned + serde::Serialize,
    V: std::fmt::Debug + serde::de::DeserializeOwned + serde::Serialize,
{
    pub fn from_unqlite<P: AsRef<str>>(filename: P) -> Result<Table<U, V>, LoadError> {
        let db = UnQLite::create(filename);
        let mut first = db
            .first()
            .ok_or(LoadError::DbTableError(DbTableError::DbDoesNotExist))?;

        // let (header_key, header_value) = first.key_value();

        let mut headers: Vec<String> = Vec::new();
        let mut metadata: Option<HashMap<String, String>> = None;
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
            let v: Vec<V> = bincode::deserialize(&item_v)?;
            btable.insert(k, v);
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
                let v: Vec<V> = bincode::deserialize(&item_v)?;
                btable.insert(k, v);
            }
            first = cursor;
        }
        if headers == Vec::<String>::new() {
            return Err(LoadError::DbTableError(DbTableError::DbHeaderDoesNotExist));
        }

        let mut table = Table::new_btreemap(headers, btable);
        if let Some(x) = metadata {
            table.set_meta_data(x)
        }
        Ok(table)
    }

    pub fn save_unqlite<P: AsRef<str>>(&self, filename: P) -> Result<(), LoadError> {
        let db = UnQLite::create(filename);

        let first = match db.first(){
            Some(_) => return Err(LoadError::DbTableError(DbTableError::DbExists)),
            None => ()
        };

        db.kv_store(
            bincode::serialize("__HEADER")?,
            bincode::serialize(&self.headers)?,
        )?;

        for (k, v) in self.iter() {
            db.kv_store(
                bincode::serialize(k)?,
                bincode::serialize(v)?,
            )?;
        }
        Ok(())
    }

    pub fn save_unqlite_override<P: AsRef<str>>(&self, filename: P) -> Result<(), LoadError> {
        match self.save_unqlite(&filename){
            Ok(x) => return Ok(x),
            Err(LoadError::DbTableError(DbTableError::DbExists)) => {
                self.delete_unqlite(&filename)?;
                self.save_unqlite(&filename)?;
            },
            Err(e) => return Err(e)
        };
        Ok(())
    }

    pub fn delete_unqlite<P: AsRef<str>>(&self, filename: P) -> Result<(), LoadError> {
        let db = UnQLite::create(filename);
        let mut first = match db.first(){
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
