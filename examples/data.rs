extern crate chrono;
extern crate csv;
use chrono::NaiveDateTime;
use timeseries::{Table, TableMetaTrait};

const TSV_DATA: &'static str = r#"d87datum	d87zweig	d87syst	d87av_anz	d87vl_anz	d50bezeich	d12sylang
2010-02-26 00:00:00.000	0	2428	1	0	Centrale	Adult
2010-03-04 00:00:00.000	0	2401	1	0	Centrale	Adult
2010-03-05 00:00:00.000	0	2437	1	0	Centrale	Adult
2010-03-10 00:00:00.000	0	1400	1	0	Centrale	Junior
2010-03-16 00:00:00.000	0	2402	1	0	Centrale	Adult
2010-04-06 00:00:00.000	0	2316	0	1	Centrale	Adult
2010-04-23 00:00:00.000	0	3150	10	0	Centrale	Junior
2010-04-28 00:00:00.000	0	2190	1	0	Centrale	Adult
2010-05-03 00:00:00.000	0	1601	0	1	Centrale	Junior
2010-05-11 00:00:00.000	0	2907	1	0	Centrale	Junior
"#;

fn main() {
    let mut rdr = csv::ReaderBuilder::default()
        .delimiter(b'\t')
        .has_headers(true)
        .from_reader(TSV_DATA.as_bytes());

    let csv_header = rdr.headers().unwrap();
    let mut header_iter = csv_header.iter().map(|x| x.to_string());
    let time_column = header_iter.next().unwrap();
    let header = header_iter.collect();

    let mut indexes = vec![];
    let mut data = vec![];
    for result in rdr.records() {
        let record = result.unwrap().clone();
        let mut first = true;
        let mut row: Vec<String> = vec![];
        for v in record.iter() {
            if first {
                indexes.push(NaiveDateTime::parse_from_str(v, "%F %H:%M:%S%.3f").unwrap());
                first = false;
            } else {
                row.push(v.to_string())
            }
        }
        data.push(row);
    }

    let mut t = Table::new(header, indexes, data).unwrap();
    t.set_meta_key(String::from("time_column"), time_column);
    println!("{:#?}", t);
}
