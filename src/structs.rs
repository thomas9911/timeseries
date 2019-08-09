use std::collections::BTreeMap;
use crate::TableError;
use chrono::Datelike;


#[derive(Debug, PartialEq)]
pub struct Table<U, V>
where
    U: Datelike + std::cmp::Ord,
{
    headers: Vec<String>,
    data: BTreeMap<U, Vec<V>>,
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

#[cfg(test)]
mod array_test{
    use crate::Table;
    use crate::{s, dt, vec2};
    use std::collections::BTreeMap;

    #[test]
    fn table_is_same_with_btreemap() {
        let now: chrono::DateTime<chrono::FixedOffset> = chrono::Utc::now().into();
        let headers = vec![s!("h1"), s!("h2")];

        let times = vec![dt!("2019-01-01T12:00:00Z").unwrap(), now.clone()];
        let d = vec2![["Hark", "Bark"], ["Hans", "kaas"]];

        let t1 = Table::new(headers.clone(), times, d).unwrap();

        let mut d = BTreeMap::new();
        d.insert(dt!("2019-01-01T12:00:00Z").unwrap(), vec!["Hark", "Bark"]);
        d.insert(now, vec!["Hans", "kaas"]);

        let t2 = Table::new_btreemap(headers, d);

        assert_eq!(t1, t2);
    }
}