use timeseries;
use timeseries::{vec2, BtreeMapTrait, Table, TableMetaTrait};

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
    for i in 1..25000 {
        indexes.push(i as u16);
        let row: Vec<i32> = vec![i * 10, i * 7 + 2, 25, i.pow(2)];
        d.push(row);
    }

    Table::new(headers, indexes, d).unwrap()
}

fn main() {
    let table = new_table_data();

    table.save_unqlite_override("./cheeses").unwrap();

    let t1: Table<u16, i32> = Table::from_unqlite("./cheeses").unwrap();

    println!("{:?}",t1);
}
