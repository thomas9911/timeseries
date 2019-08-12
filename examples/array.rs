use std::collections::BTreeMap;
use chrono::{DateTime, FixedOffset, };
use timeseries::{Table, vec2};

macro_rules! s {
    ($t:expr) => {
        String::from($t)
    };
}

macro_rules! dt {
    ($t:expr) => {
        chrono::DateTime::parse_from_rfc3339($t)
    };
}



fn main() {
    let now: DateTime<FixedOffset> = chrono::Utc::now().into();
    let headers = vec![s!("h1"), s!("h2")];
    let times = vec![dt!("2019-01-01T12:00:00Z").unwrap(), now.clone()];
    let d = vec2![["Hark", "Bark"], ["Hans", "kaas"],];

    let t1 = Table::new(headers.clone(), times, d).unwrap();

    let mut d = BTreeMap::new();
    d.insert(dt!("2019-01-01T12:00:00Z").unwrap(), vec!["Hark", "Bark"]);
    d.insert(now, vec!["Hans", "kaas"]);

    let t2 = Table::new_btreemap(headers, d);

    println!("{:?}", t1);
    println!("{:?}", t2);
    println!("Tables are the same: {}", t1 == t2);
}
