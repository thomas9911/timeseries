#![cfg(feature = "ndarray")]

use std::collections::BTreeMap;
use crate::TableError;
use chrono::Datelike;


#[derive(Debug, PartialEq)]
pub struct NdTable<U, V, D>
where
    U: Datelike + std::cmp::Ord,
    D: ndarray::Dimension
{
    headers: Vec<String>,
    data: BTreeMap<U, ndarray::Array<V, <D as ndarray::Dimension>::Smaller>>,
}

impl<U, V, D> NdTable<U, V, D>
where
    U: Datelike + std::cmp::Ord,
    D: ndarray::Dimension + ndarray::RemoveAxis,
    V: Clone
{
    pub fn new(
        headers: Vec<String>,
        time_data: Vec<U>,
        data: ndarray::Array<V, D>,
    ) -> Result<NdTable<U, V, D>, TableError> {
        let data = Self::to_btreemap(time_data, data)?;
        // Ok(Table::new_btreemap(headers, data))
        Ok(NdTable{
            headers,
            data
        })
    }

    fn to_btreemap(
        time_data: Vec<U>,
        data: ndarray::Array<V, D>,
    ) -> Result<BTreeMap<U, ndarray::Array<V, <D as ndarray::Dimension>::Smaller>>, TableError>{
        match D::NDIM{
            Some(0) | Some(1) => return Err(TableError::new("Array should have a dimension of two or higher")),
            Some(_) => (),
            None => return Err(TableError::new("Array should have a fixed dimension")),
        };

        let mut tree_data: BTreeMap<U, ndarray::Array<V, <D as ndarray::Dimension>::Smaller>> = BTreeMap::new();
        if data.len_of(ndarray::Axis{0: 0}) != time_data.len() {
            return Err(TableError::new("time and data length should be equal"));
        }

        for (k, v) in time_data.into_iter().zip(data.outer_iter()) {
            tree_data.insert(k, v.into_owned());
        }
        Ok(tree_data)
    }
}



#[cfg(all(test, feature = "ndarray"))]
mod ndarray_test{
    use crate::NdTable;
    use crate::{s, dt};

    #[test]
    fn ndarray_table(){
        let now: chrono::DateTime<chrono::FixedOffset> = chrono::Utc::now().into();
        let headers = vec![s!("h1"), s!("h2")];

        let times = vec![dt!("2019-01-01T12:00:00Z").unwrap(), now.clone(), dt!("2019-01-05T12:00:00Z").unwrap()];
        let d = ndarray::array![["Hark", "Bark"], ["Hans", "kaas"], ["Hans", "kaas"]];

        let t1 = NdTable::new(headers, times, d);
        assert!(t1.is_ok());
    }
}