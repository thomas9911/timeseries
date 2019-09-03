#![cfg(feature = "sqlite_db")]
use crate::{BtreeMapTrait, DbObject, DbTableError, Table, TableMetaTrait};
use rusqlite::{Connection, OpenFlags, ToSql, NO_PARAMS};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug)]
pub enum SqliteError {
    Bincode(std::boxed::Box<bincode::ErrorKind>),
    DbTableError(DbTableError),
    Sqlite(rusqlite::Error),
    Io(std::io::Error),
}

impl std::fmt::Display for SqliteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<rusqlite::Error> for SqliteError {
    fn from(err: rusqlite::Error) -> SqliteError {
        SqliteError::Sqlite(err)
    }
}

impl From<std::boxed::Box<bincode::ErrorKind>> for SqliteError {
    fn from(err: std::boxed::Box<bincode::ErrorKind>) -> SqliteError {
        SqliteError::Bincode(err)
    }
}

impl From<std::io::Error> for SqliteError {
    fn from(err: std::io::Error) -> SqliteError {
        SqliteError::Io(err)
    }
}

#[derive(Debug, Clone)]
pub struct SqliteConfig {
    pub location: std::path::PathBuf,
    pub flags: OpenFlags,
}

impl SqliteConfig {
    pub fn new<P>(path: P) -> SqliteConfig
    where
        P: Into<std::path::PathBuf>,
    {
        SqliteConfig {
            location: path.into(),
            flags: OpenFlags::default(),
        }
    }

    pub fn new_with_flags<P>(path: P, flags: OpenFlags) -> SqliteConfig
    where
        P: Into<std::path::PathBuf>,
    {
        SqliteConfig {
            location: path.into(),
            flags: flags,
        }
    }
}

impl<U, V> Table<U, V>
where
    U: std::fmt::Debug + Clone + std::cmp::Ord + serde::de::DeserializeOwned + serde::Serialize,
    V: std::fmt::Debug + Clone + serde::de::DeserializeOwned + serde::Serialize,
{
    /// creates tables
    pub fn init_sqlite(&self, config: &SqliteConfig) -> Result<(), SqliteError> {
        let connection = Self::connect_sqlite(config)?;
        connection.execute(
            "CREATE TABLE row (
                    id              INTEGER NOT NULL PRIMARY KEY,
                    key             BLOB NOT NULL UNIQUE,
                    item            BLOB NOT NULL,
                    hash            INTEGER NOT NULL
                  )",
            NO_PARAMS,
        )?;
        Ok(())
    }

    /// removes all data and tables
    pub fn uninit_sqlite(&self, config: &SqliteConfig) -> Result<(), SqliteError> {
        let connection = Self::connect_sqlite(config)?;
        connection.execute("DROP TABLE IF EXISTS row", NO_PARAMS)?;
        Ok(())
    }

    pub fn remove_sqlite(&self, config: &SqliteConfig) -> Result<(), SqliteError> {
        std::fs::remove_file(&config.location)?;
        Ok(())
    }

    pub fn connect_sqlite(config: &SqliteConfig) -> Result<Connection, SqliteError> {
        Ok(Connection::open_with_flags(
            &config.location,
            config.flags.to_owned(),
        )?)
    }

    pub fn save_sqlite(&self, config: &SqliteConfig) -> Result<(), SqliteError> {
        let connection = Self::connect_sqlite(config)?;
        let mut stmt =
            connection.prepare("INSERT INTO row (key, item, hash) VALUES (?1, ?2, ?3)")?;

        let tmp = DbObject::new(self.headers.clone());
        let data: &[&ToSql] = &[
            &bincode::serialize("__HEADER")?,
            &bincode::serialize(&tmp.item)?,
            &(tmp.hash as i64),
        ];
        stmt.execute(data)?;

        let tmp = DbObject::new(self.meta_data.clone());
        let data: &[&ToSql] = &[
            &bincode::serialize("__META_DATA")?,
            &bincode::serialize(&tmp.item)?,
            &(tmp.hash as i64),
        ];
        stmt.execute(data)?;

        for (k, v) in self.iter() {
            let tmp = DbObject::new(v.clone());
            let data: &[&ToSql] = &[
                &bincode::serialize(k)?,
                &bincode::serialize(&tmp.item)?,
                &(tmp.hash as i64),
            ];
            stmt.execute(data)?;
        }
        Ok(())
    }

    pub fn update_sqlite(&self, config: &SqliteConfig) -> Result<(), SqliteError> {
        let connection = Self::connect_sqlite(config)?;
        let mut insert_stmt =
            connection.prepare("INSERT INTO row (key, item, hash) VALUES (?1, ?2, ?3)")?;
        let mut update_stmt =
            connection.prepare("UPDATE row SET item = ?2, hash = ?3 WHERE key = ?1")?;
        let mut count_stmt =
            connection.prepare("SELECT count(*) from row WHERE key = ?1 AND hash = ?2")?;
        let mut exist_stmt = connection.prepare("SELECT count(*) from row WHERE key = ?1")?;

        let tmp = DbObject::new(self.headers.clone());
        let data: &[&ToSql] = &[
            &bincode::serialize("__HEADER")?,
            &bincode::serialize(&tmp.item)?,
            &(tmp.hash as i64),
        ];

        if count_stmt.query_row(&[data[0], data[2]], |row| row.get::<_, i64>(0))? == 0 {
            match exist_stmt.query_row(&[data[0]], |row| row.get::<_, i64>(0))? {
                0 => {
                    insert_stmt.execute(data)?;
                }
                1 => {
                    update_stmt.execute(data)?;
                }
                _ => unreachable!(),
            }
        }

        let tmp = DbObject::new(self.meta_data.clone());
        let data: &[&ToSql] = &[
            &bincode::serialize("__META_DATA")?,
            &bincode::serialize(&tmp.item)?,
            &(tmp.hash as i64),
        ];

        if count_stmt.query_row(&[data[0], data[2]], |row| row.get::<_, i64>(0))? == 0 {
            match exist_stmt.query_row(&[data[0]], |row| row.get::<_, i64>(0))? {
                0 => {
                    insert_stmt.execute(data)?;
                }
                1 => {
                    update_stmt.execute(data)?;
                }
                _ => unreachable!(),
            }
        }

        for (k, v) in self.iter() {
            let tmp = DbObject::new(v.clone());
            let data: &[&ToSql] = &[
                &bincode::serialize(k)?,
                &bincode::serialize(&tmp.item)?,
                &(tmp.hash as i64),
            ];

            if count_stmt.query_row(&[data[0], data[2]], |row| row.get::<_, i64>(0))? == 0 {
                match exist_stmt.query_row(&[data[0]], |row| row.get::<_, i64>(0))? {
                    0 => {
                        insert_stmt.execute(data)?;
                    }
                    1 => {
                        update_stmt.execute(data)?;
                    }
                    _ => unreachable!(),
                }
            }
        }
        Ok(())
    }

    pub fn from_sqlite(config: &SqliteConfig) -> Result<Table<U, V>, SqliteError> {
        let connection = Self::connect_sqlite(config)?;

        let mut headers: Vec<String> = Vec::new();
        let mut metadata: Option<HashMap<String, String>> = None;
        let mut btable = BTreeMap::new();

        let mut stmt = connection.prepare("SELECT key, item FROM row")?;
        let mut rows = stmt.query(NO_PARAMS)?;
        while let Some(row) = rows.next()? {
            let item_k: Vec<u8> = row.get(0)?;
            let item_v: Vec<u8> = row.get(1)?;
            let mut n = true;

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
                // let v: Vec<V> = bincode::deserialize(&item_v)?;
                btable.insert(k, v);
            }
        }
        if headers == Vec::<String>::new() {
            return Err(SqliteError::DbTableError(
                DbTableError::DbHeaderDoesNotExist,
            ));
        }

        let mut table = Table::new_btreemap(headers, btable);
        if let Some(x) = metadata {
            table.set_meta_data(x)
        }
        Ok(table)
    }
}
