use std::fs::File;

use csv::{ReaderBuilder, StringRecord};

fn main() -> std::io::Result<()> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(File::open("books.csv")?);

    println!("{}", fmt(reader.headers()?));
    for record in reader.records() {
        println!("{}", fmt(&record?));
    }

    Ok(())
}

fn fmt(rec: &StringRecord) -> String {
    rec.into_iter().map(|v| format!("{:20}", v)).collect()
}
