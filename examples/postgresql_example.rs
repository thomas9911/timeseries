use timeseries;
use timeseries::postgres;
use timeseries::{BtreeMapTrait, PostgresConfig, Table};

fn new_table_data() -> Table<u16, i32> {
    let headers = vec![
        String::from("p10"),
        String::from("data"),
        String::from("twentyfive"),
        String::from("squares"),
    ];

    // let indexes = vec![1, 2, 3, ...];
    // let d = vec2![
    //     [10, 09, 25, 01],
    //     [20, 16, 25, 04],
    //     [30, 23, 25, 09],
    //          .
    //          .
    //          .
    // ];

    let mut indexes = Vec::new();
    let mut d = Vec::new();
    for i in 1..2500 {
        indexes.push(i as u16);
        let row: Vec<i32> = vec![i * 10, i * 7 + 2, 25, i.pow(2)];
        d.push(row);
    }

    Table::new(headers, indexes, d).unwrap()
}

fn main() {
    let postgres_ip = "postgres://postgres@192.168.99.100:5432/postgres";
    let config = PostgresConfig::new(postgres_ip, postgres::TlsMode::None).unwrap();
    let mut table = new_table_data();

    // <Table<u16, i32>>::connect_postgresql(config).unwrap();

    // prepares the postgres database
    table.init_postgresql(config).unwrap();
    let config = PostgresConfig::new(postgres_ip, postgres::TlsMode::None).unwrap();

    // saves the whole table to the database
    table.save_postgresql(config).unwrap();

    table.insert(3000, vec![30000, 123456, 25, 900000]);

    let config = PostgresConfig::new(postgres_ip, postgres::TlsMode::None).unwrap();
    // only update the keys that are different
    table.update_postgresql(config).unwrap();

    let config = PostgresConfig::new(postgres_ip, postgres::TlsMode::None).unwrap();
    // get the table back from the postgres database
    let t1: Table<u16, i32> = Table::from_postgresql(config).unwrap();

    let config = PostgresConfig::new(postgres_ip, postgres::TlsMode::None).unwrap();
    // removes all the tables from the database. The inverse of init.
    table.uninit_postgresql(config).unwrap();

    if table != t1 {
        println!("tables are not the same");
    };
}
