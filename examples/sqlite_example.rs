use timeseries;
use timeseries::{BtreeMapTrait, SqliteConfig, Table};

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
    for i in 1..25 {
        indexes.push(i as u16);
        let row: Vec<i32> = vec![i * 10, i * 7 + 2, 25, i.pow(2)];
        d.push(row);
    }

    Table::new(headers, indexes, d).unwrap()
}

fn main() {
    let sqlite_path = "temp.db";
    let config = SqliteConfig::new(sqlite_path);
    let mut table = new_table_data();

    // prepares the sqlite database and creates file if needed
    table.init_sqlite(&config).unwrap();

    // saves the whole table to the database
    table.save_sqlite(&config).unwrap();

    table.insert(3000, vec![30000, 123456, 25, 900000]);

    // only update the keys that are different
    table.update_sqlite(&config).unwrap();

    // get the table back from the postgres database
    let t1: Table<u16, i32> = Table::from_sqlite(&config).unwrap();

    // removes all the tables from the database. The inverse of init.
    table.uninit_sqlite(&config).unwrap();

    // removes the database file
    table.remove_sqlite(&config).unwrap();

    if table != t1 {
        println!("tables are not the same");
    };
}
