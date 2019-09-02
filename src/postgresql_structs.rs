#![cfg(feature = "postgresql_db")]
use crate::{BtreeMapTrait, DbObject, DbTableError, Table, TableMetaTrait};
use postgres::types::ToSql;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug)]
pub enum PostgresqlError {
    Bincode(std::boxed::Box<bincode::ErrorKind>),
    DbTableError(DbTableError),
    Postgres(postgres::Error),
}

impl std::fmt::Display for PostgresqlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<postgres::Error> for PostgresqlError {
    fn from(err: postgres::Error) -> PostgresqlError {
        PostgresqlError::Postgres(err)
    }
}

impl From<std::boxed::Box<bincode::ErrorKind>> for PostgresqlError {
    fn from(err: std::boxed::Box<bincode::ErrorKind>) -> PostgresqlError {
        PostgresqlError::Bincode(err)
    }
}

#[derive(Debug)]
pub struct PostgresConfig<'a> {
    pub config: postgres::params::ConnectParams,
    pub tls: postgres::TlsMode<'a>,
}

impl<'a> PostgresConfig<'a> {
    pub fn new<S>(
        str_config: S,
        tls_config: postgres::TlsMode<'a>,
    ) -> Result<PostgresConfig, Box<std::error::Error + Sync + Send>>
    where
        S: postgres::params::IntoConnectParams,
    {
        Ok(PostgresConfig {
            config: str_config.into_connect_params()?,
            tls: tls_config,
        })
    }
}

impl<U, V> Table<U, V>
where
    U: std::fmt::Debug + Clone + std::cmp::Ord + serde::de::DeserializeOwned + serde::Serialize,
    V: std::fmt::Debug + Clone + serde::de::DeserializeOwned + serde::Serialize,
{
    /// creates tables
    pub fn init_postgresql(&self, config: PostgresConfig) -> Result<(), PostgresqlError> {
        let connection = Self::connect_postgresql(config)?;
        connection.execute(
            "CREATE TABLE row (
                    id              SERIAL PRIMARY KEY,
                    key             BYTEA NOT NULL,
                    item            BYTEA NOT NULL,
                    hash            BIGINT NOT NULL,
                    UNIQUE(key)
                  )",
            &[],
        )?;
        Ok(())
    }

    /// removes all data and tables
    pub fn uninit_postgresql(&self, config: PostgresConfig) -> Result<(), PostgresqlError> {
        let connection = Self::connect_postgresql(config)?;
        connection.execute("DROP TABLE IF EXISTS row CASCADE", &[])?;
        Ok(())
    }

    pub fn connect_postgresql(
        config: PostgresConfig,
    ) -> Result<postgres::Connection, PostgresqlError> {
        Ok(postgres::Connection::connect(config.config, config.tls)?)
    }

    pub fn save_postgresql(&self, config: PostgresConfig) -> Result<(), PostgresqlError> {
        let connection = Self::connect_postgresql(config)?;
        let stmt = connection.prepare("INSERT INTO row (key, item, hash) VALUES ($1, $2, $3)")?;

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

    pub fn from_postgresql(config: PostgresConfig) -> Result<Table<U, V>, PostgresqlError> {
        let connection = Self::connect_postgresql(config)?;

        let mut headers: Vec<String> = Vec::new();
        let mut metadata: Option<HashMap<String, String>> = None;
        let mut btable = BTreeMap::new();

        for row in &connection.query("SELECT key, item FROM row", &[])? {
            let item_k: Vec<u8> = row.get(0);
            let item_v: Vec<u8> = row.get(1);
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
            return Err(PostgresqlError::DbTableError(
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
