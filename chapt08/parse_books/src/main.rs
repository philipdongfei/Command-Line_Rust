use csv::{ReaderBuilder, StringRecord};
use std::{
    fs::File,
    ops::Range,
};



fn main() -> std::io::Result<()> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(File::open("books.csv")?);

    println!("{}", fmt(reader.headers()?));
    let range: Vec::<Range<usize>> = [(0usize..1usize), (1usize..2usize)].to_vec();
    for record in reader.records() {
        println!("{}", fmt(&record?));
    }

    let record = StringRecord::from(vec!["foo", "quux", "z"]);
    //let range = record.range(1).expect("a record range");
    println!("range: {:?}", extract_fields(&record, &range));

    Ok(())
}

fn fmt(rec: &StringRecord) -> String {
    rec.into_iter().map(|v| format!("{:20}", v)).collect()
}

fn extract_fields(record: &StringRecord, field_pos: &[Range<usize>]) -> Vec<String>
{
    let mut fields = Vec::<String>::new();
    for r in field_pos {
        for i in r.clone().collect::<Vec<_>>() {
            match record.get(i) {
                None => (),
                Some(f) => fields.push(f.to_string()),
            }        
        }
    }
    fields
}
