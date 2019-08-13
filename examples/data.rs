use timeseries::chrono::{NaiveDateTime, ParseError};
use timeseries::csv;
use timeseries::read_csv_to_datetable;

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

fn parse_date(v: &str) -> Result<NaiveDateTime, YourError> {
    match NaiveDateTime::parse_from_str(v, "%F %H:%M:%S%.3f") {
        Ok(x) => Ok(x),
        Err(e) => Err(YourError::new(format!("{}", e))),
    }
}

#[derive(Debug)]
pub struct YourError(String);

impl YourError {
    pub fn new<S: Into<String>>(s: S) -> YourError {
        YourError { 0: s.into() }
    }
}

impl std::fmt::Display for YourError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    let mut rdr = csv::ReaderBuilder::default()
        .delimiter(b'\t')
        .has_headers(true)
        .from_reader(TSV_DATA.as_bytes());
    let y = read_csv_to_datetable(&mut rdr, parse_date).unwrap();

    let mut rdr = csv::ReaderBuilder::default()
        .delimiter(b'\t')
        .has_headers(true)
        .from_reader(TSV_DATA.as_bytes());
    let t = read_csv_to_datetable(&mut rdr, |x| -> Result<_, ParseError> {
        Ok(NaiveDateTime::parse_from_str(x, "%F %H:%M:%S%.3f")?)
    }).unwrap();

    assert_eq!(y, t);
    println!("{:#?}", t);
}
