#![cfg(feature = "redis_db")]
use crate::{BtreeMapTrait, DbObject, DbTableError, Table, TableMetaTrait};
use std::collections::{BTreeMap, HashMap};

use redis::{Client, Commands, Connection, ConnectionInfo, IntoConnectionInfo};

#[derive(Debug)]
pub enum RedisError {
    Bincode(std::boxed::Box<bincode::ErrorKind>),
    DbTableError(DbTableError),
    Redis(redis::RedisError),
}

impl std::fmt::Display for RedisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<redis::RedisError> for RedisError {
    fn from(err: redis::RedisError) -> RedisError {
        RedisError::Redis(err)
    }
}

impl From<std::boxed::Box<bincode::ErrorKind>> for RedisError {
    fn from(err: std::boxed::Box<bincode::ErrorKind>) -> RedisError {
        RedisError::Bincode(err)
    }
}

#[derive(Debug, Clone)]
pub struct RedisConfig {
    pub info: ConnectionInfo,
}

impl RedisConfig {
    pub fn new<T>(info: T) -> Result<RedisConfig, RedisError>
    where
        T: IntoConnectionInfo,
    {
        Ok(RedisConfig {
            info: info.into_connection_info()?,
        })
    }
}

impl<U, V> Table<U, V>
where
    U: std::fmt::Debug + Clone + std::cmp::Ord + serde::de::DeserializeOwned + serde::Serialize,
    V: std::fmt::Debug + Clone + serde::de::DeserializeOwned + serde::Serialize,
{
    pub fn connect_redis(config: &RedisConfig) -> Result<Connection, RedisError> {
        let client = Client::open(config.info.to_owned())?;
        let con = client.get_connection()?;
        Ok(con)
    }
    pub fn from_redis(config: &RedisConfig) -> Result<Table<U, V>, RedisError> {
        let mut connection = Self::connect_redis(config)?;
        let mut connection2 = Self::connect_redis(config)?;
        let mut btreemap = BTreeMap::new();
        let mut headers: DbObject<Vec<String>> = DbObject::new(vec![]);
        let mut meta_data: DbObject<Option<HashMap<String, String>>> = DbObject::new(None);

        for item in connection.scan::<Vec<u8>>()? {
            let mut n = true;
            let row_data: Vec<u8> = connection2.get(item.clone())?;
            if let Ok(x) = bincode::deserialize::<String>(&item) {
                if x == "__HEADER" {
                    headers = bincode::deserialize(&row_data)?;
                    n = false;
                }
                if x == "__META_DATA" {
                    meta_data = bincode::deserialize(&row_data)?;
                    n = false;
                }
            }
            if n {
                let key: U = bincode::deserialize(&item)?;
                let row: DbObject<Vec<V>> = bincode::deserialize(&row_data)?;
                btreemap.insert(key, row.item);
            }
        }

        let mut table = Table::new_btreemap(headers.item, btreemap);
        if let Some(x) = meta_data.item {
            table.set_meta_data(x)
        }
        Ok(table)
    }
    pub fn save_redis(&self, config: &RedisConfig) -> Result<(), RedisError> {
        let mut connection = Self::connect_redis(config)?;

        let has_header: bool = connection.exists(bincode::serialize("__HEADER")?)?;
        if has_header {
            return Err(RedisError::DbTableError(DbTableError::DbExists));
        }

        connection.set(
            bincode::serialize("__HEADER")?,
            bincode::serialize(&DbObject::new(self.headers.clone()))?,
        )?;

        connection.set(
            bincode::serialize("__META_DATA")?,
            bincode::serialize(&DbObject::new(self.meta_data.clone()))?,
        )?;

        for (k, v) in self.iter() {
            let tmp = DbObject::new(v.clone());
            connection.set(bincode::serialize(k)?, bincode::serialize(&tmp)?)?;
        }
        Ok(())
    }
    pub fn update_redis(&self, config: &RedisConfig) -> Result<(), RedisError> {
        let mut connection = Self::connect_redis(config)?;
        let mut headers: DbObject<Vec<String>> = DbObject::new(vec![]);
        let mut meta_data: DbObject<Option<HashMap<String, String>>> = DbObject::new(None);
        for (k, v) in self.iter() {
            let tmp = DbObject::new(v.clone());
            let key = bincode::serialize(k)?;
            let key_exist: bool = connection.exists(key.clone())?;
            if !key_exist {
                connection.set(bincode::serialize(k)?, bincode::serialize(&tmp)?)?;
            } else {
                let db_value: Vec<u8> = connection.get(key)?;
                let db_object: DbObject<Vec<V>> = bincode::deserialize(&db_value)?;
                if tmp != db_object {
                    connection.set(bincode::serialize(k)?, bincode::serialize(&tmp)?)?;
                }
            }
        }
        Ok(())
    }
    pub fn delete_redis(&self, config: &RedisConfig) -> Result<(), RedisError> {
        let mut connection = Self::connect_redis(config)?;
        redis::cmd("FLUSHDB").query(&mut connection)?;
        Ok(())
    }
}
