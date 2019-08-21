#[cfg(feature = "csv")]
extern crate csv;

#[cfg(feature = "chrono")]
extern crate chrono;

#[cfg(all(feature = "chrono", feature = "csv"))]
use crate::{Table, TableMetaTrait, TableReadError};

#[cfg(all(feature = "chrono", feature = "csv"))]
pub fn read_csv_to_datetable<R, U, F, Y>(
    rdr: &mut csv::Reader<R>,
    index_parse: F,
) -> Result<Table<U, String>, TableReadError>
where
    R: std::io::Read,
    U: std::fmt::Debug + std::cmp::Ord,
    F: Fn(&str) -> Result<U, Y>,
    Y: std::fmt::Display,
{
    let csv_header = rdr.headers()?;
    let mut header_iter = csv_header.iter().map(|x| x.to_string());
    let time_column = header_iter.next().expect("no header found");
    let header = header_iter.collect();

    let mut indexes = vec![];
    let mut data = vec![];
    for result in rdr.records() {
        let record = result?.to_owned();
        let mut first = true;
        let mut row: Vec<String> = vec![];
        for v in record.iter() {
            if first {
                let i = match index_parse(v) {
                    Ok(x) => x,
                    Err(e) => return Err(TableReadError::new(format!("{}", e))),
                };
                indexes.push(i);
                first = false;
            } else {
                row.push(v.to_string())
            }
        }
        data.push(row);
    }

    let mut t = Table::new(header, indexes, data)?;
    t.set_meta_key(String::from("time_column"), time_column);
    Ok(t)
}
