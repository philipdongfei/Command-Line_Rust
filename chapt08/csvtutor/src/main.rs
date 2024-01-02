use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::io;
use std::process;
use serde::{Deserialize, Serialize};

/*
// tutorial-read-01
fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    //let file = File::open(file_path)?;
    //let mut rdr = csv::Reader::from_reader(file);
    let mut rdr = csv::Reader::from_path(file_path)?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
*/

/*
//tutorial-read-headers-01
fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin());
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
*/
/*
//tutorial-read-headers-02
fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    {
        // We nest this call in its own scope because of lifetimes.
        let headers = rdr.headers()?; // rdr.headers()?.clone();
        println!("{:?}", headers);
    }
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }

    // We can ask for the headers at any time. There's no need to nest this
    // call in its own scope because we never try to borrow the reader again.
    let headers = rdr.headers()?;
    println!("{:?}", headers);
    Ok(())
}
*/

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

/*
//tutorial-read-delimiter-01.rs
fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .double_quote(false)
        .escape(Some(b'\\'))
        .flexible(true)
        .comment(Some(b'#'))
        .from_reader(io::stdin());
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
*/
/*
//tutorial-read-serde-01.rs
fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        let record = result?;

        let city = &record[0];
        let state = &record[1];
        // Some records are missing population counts, so if we can't
        // parse a number, treat the population count as missing instead
        // of returning an error.
        let pop: Option<u64> = record[2].parse().ok();
        // Lucky us! Latitudes and longitudes are available for every record.
        // Therefore, if one couldn't be parsed, return an error.
        let latitude: f64 = record[3].parse()?;
        let longitude: f64 = record[4].parse()?;

        println!(
            "city: {:?}, state: {:?}, \
             pop: {:?}, latitude: {:?}, longitude: {:?}",
             city, state, pop, latitude, longitude
        );

    }
    Ok(())
}
*/
/*
//tutorial-read-serde-02
// This introduces a type alias so that we can conveniently reference our
// record type.
type Record = (String, String, Option<u64>, f64, f64);

fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    // Instead of creating an iterator with the `records` method, we create
    // an iterator with the `deserialize` method.
    for result in rdr.deserialize() {
        // We must tell Serde what type we want to deserialize into.
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
*/

/*
//tutorial-read-serde-03.rs
use std::collections::HashMap;

// This introduces a type alias so that we can conveniently reference our
// record type.
type Record = HashMap<String, String>;

fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
*/

/*
//tutorial-read-serde-04.rs
//
// This lets us write `#[derive(Deserialize)]`.
use serde::Deserialize;


// We don't need to derive `Debug` (which doesn't require Serde), but it's a
// good habit to do it for all your types.
//
// Notice that the field names in this struct are NOT in the same order as
// the fields in the CSV data!
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    latitude: f64,
    longitude: f64,
    #[serde(deserialize_with = "csv::invalid_option")]
    population: Option<u64>,
    city: String, 
    state: String,
}



#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Row {
    country: String,
    city: String,
    accent_city: String,
    region: String,
    population: Option<u64>,
    latitude: Option<f64>,
    longitude: Option<f64>,
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        let record: Record = result?;
        //let record: Row = result?;
        println!("{:?}", record);
        // Try this if you don't like each record smushed on one line:
        // println!("{:#?}", record);
    }
    Ok(())
}
*/
/*
//tutorial-read-serde-invalid-01.rs
fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
*/

/*
//tutorial-write-01.rs
//use std::{error::Error, io, process};

fn run() -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());
    // Since we're writing records manually, we must explicitly write our
    // header record. A header record is written the same way that other
    // records are written.
    /*
    wtr.write_record(&["City", "State", "Population", "Latitude", "Longitude"])?;
    wtr.write_record(&["Davidsons Landing", "AK", "", "65.2419444", "-165.2716667"])?;
    wtr.write_record(&["Kenai", "AK", "7610", "60.5544444", "-151.2583333"])?;
    wtr.write_record(&["Oakman", "AL", "", "33.7133333","-87.3886111"])?;
    */
    wtr.write_record(&["field 1", "field 2", "etc"])?;
    // A slice of byte strings.
    wtr.write_record(&[b"a", b"b", b"c"])?;
    // A vector
    wtr.write_record(vec!["a", "b", "c"])?;
    // A string record
    wtr.write_record(&csv::StringRecord::from(vec!["a", "b", "c"]))?;
    // A byte record
    wtr.write_record(&csv::ByteRecord::from(vec!["a", "b", "c"]))?;

    // A CSV writer maintains an internal buffer, so it's important
    // to flush the buffer when you're done.
    wtr.flush()?;
    Ok(())
}
*/
/*
//tutorial-write-02.rs
fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut wtr = csv::Writer::from_path(file_path)?;

    wtr.write_record(&["City", "State", "Population", "Latitude", "Longitude"])?;
    wtr.write_record(&["Davidsons Landing", "AK", "", "65.2419444", "-165.2716667"])?;
    wtr.write_record(&["Kenai", "AK", "7610", "60.5544444", "-151.2583333"])?;
    wtr.write_record(&["Oakman", "AL", "", "33.7133333","-87.3886111"])?;

    wtr.flush()?;
    Ok(())
}
*/
/*
//tutorial-write-delimiter-01.rs
fn run() -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .quote_style(csv::QuoteStyle::NonNumeric)
        .from_writer(io::stdout());

    wtr.write_record(&["City", "State", "Population", "Latitude", "Longitude"])?;
    wtr.write_record(&["Davidsons Landing", "AK", "", "65.2419444", "-165.2716667"])?;
    wtr.write_record(&["Kenai", "AK", "7610", "60.5544444", "-151.2583333"])?;
    wtr.write_record(&["Oakman", "AL", "", "33.7133333","-87.3886111"])?;

    wtr.flush()?;
    Ok(())
}
*/
/*
//tutorial-write-serde-01.rs
fn run() -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    // We still need to write headers manually
    wtr.write_record(&["City", "State", "Population", "Latitude", "Longitude"])?;

    // But now we can write records by providing a normal Rust value.
    //
    // Note that the odd `None::<u64>` syntax is required because `None` on
    // its own doesn't have a concrete type, but Serde needs a concrete type
    // in order to serialize it. That is, `None` has type `Option<T>` but
    // `None::<u64>` has type `Option<u64>`.
    wtr.serialize(("Davidsons Landing", "AK", None::<u64>, "65.2419444", "-165.2716667"))?;
    wtr.serialize(("Kenai", "AK", Some(7610), "60.5544444", "-151.2583333"))?;
    wtr.serialize(("Oakman", "AL", None::<u64>, "33.7133333","-87.3886111"))?;

    wtr.flush()?;
    Ok(())
}
*/
/*
//tutorial-write-delimiter-02.rs
// Note that structs can derive both Serialize and Deserialize!
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Record<'a> {
    city: &'a str,
    state: &'a str,
    population: Option<u64>,
    latitude: f64,
    longitude: f64,
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.serialize(Record {
        city: "Davidsons Landing",
        state: "AK",
        population: None,
        latitude: 65.2419444,
        longitude: -165.2716667,
    })?;
    wtr.serialize(Record {
        city: "Kenai",
        state: "AK",
        population: Some(7610),
        latitude: 60.5544444,
        longitude: -151.2583333,
    })?;
    wtr.serialize(Record {
        city: "Oakman",
        state: "AL",
        population: None,
        latitude: 33.7133333,
        longitude: -87.3886111,
    })?;

    wtr.flush()?;
    Ok(())
}
*/

/*
//tutorial-pipeline-search-01.rs
fn run() -> Result<(), Box<dyn Error>> {
    // Get the query from the positional arguments.
    // If one doesn't exist, return an error.
    let query = match env::args().nth(1) {
        None => return Err(From::from("expected 1 argument, but got none")),
        Some(query) => query,
    };

    println!("query: {}", query);

    // Build CSV readers and writers to stdin and stdout, respectively.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut wtr = csv::Writer::from_writer(io::stdout());

    // Before reading our data records, we should write the header record.
    wtr.write_record(rdr.headers()?)?;

    // Iterate over all the records in `rdr`, and write only records containing
    // `query` to `wtr`.
    for result in rdr.records() {
        let record = result?;
        if record.iter().any(|field| field == &query){
            wtr.write_record(&record)?;
        }
    }

    // CSV writers use an internal buffer, so we should always flush when done.
    wtr.flush()?;
    Ok(())
}
*/
/*
//tutorial-pipeline-search-02.rs
fn run() -> Result<(), Box<dyn Error>> {
    // Get the query from the positional arguments.
    // If one doesn't exist, return an error.
    let query = match env::args().nth(1) {
        None => return Err(From::from("expected 1 argument, but got none")),
        Some(query) => query,
    };

    println!("query: {}", query);

    // Build CSV readers and writers to stdin and stdout, respectively.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut wtr = csv::Writer::from_writer(io::stdout());

    // Before reading our data records, we should write the header record.
    wtr.write_record(rdr.byte_headers()?)?;

    // Iterate over all the records in `rdr`, and write only records containing
    // `query` to `wtr`.
    for result in rdr.byte_records() {
        let record = result?;
        // `query` is a `String` while `field` is now a `&[u8]`, so we'll
        // need to convert `query` to `&[u8]` before doing a comparison.
        if record.iter().any(|field| field == query.as_bytes()){
            wtr.write_record(&record)?;
        }
    }

    // CSV writers use an internal buffer, so we should always flush when done.
    wtr.flush()?;
    Ok(())
}
*/

/*
//tutorial-pipeline-pop-01.rs

// Unlike previous examples, we derive both Deserialize and Serialize. This
// means we'll be able to automatically deserialize and serialize this type.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    city: String,
    state: String,
    population: Option<u64>,
    latitude: f64,
    longitude: f64,
}

fn run() -> Result<(), Box<dyn Error>> {
    // Get the query from the positional arguments.
    // If one doesn't exist or isn't an integer, return an error.
    let minimum_pop: u64 = match env::args().nth(1) {
        None => return Err(From::from("expected 1 argument, but got none")),
        Some(arg) => arg.parse()?,
    };

    // Build CSV readers and writers to stdin and stdout, respectively.
    // Note that we don't need to write headers explicitly. Since we're
    // serializing a custom struct, that's done for us automatically.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut wtr = csv::Writer::from_writer(io::stdout());

    // Iterator over all the records in `rdr`, and write only records containing
    // a population that is greater than or equal to `minimum_pop`.
    for result in rdr.deserialize() {
        // Remember that when deserializing, we must use a type hint to 
        // indicate which type we want to deserialize our record into.
        let record: Record = result?;

        // `map_or` is a combinator on `Option`. It take two parameters:
        // a value to use when the `Option` is `None` (i.e., the record has
        // no population count) and a closure that returns another value of 
        // the same type when the `Option` is `Some`. In this case, we test it
        // against our minimum population count that we got from the command
        // line.
        if record.population.map_or(false, |pop| pop >= minimum_pop) {
            wtr.serialize(record)?;
        }
    }

    // CSV writers use an internal buffer, so we should always flush when done.
    wtr.flush()?;
    Ok(())
}
*/

/*
//tutorial-perf-alloc-01.rs
fn run() -> Result<u64, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    let mut count = 0;
    for result in rdr.records() {
        let record = result?;
        if &record[0] == "us" && &record[3] == "MA" {
            count += 1;
        }
    }
    Ok(count)
}
*/
/*
//tutorial-perf-alloc-02.rs
fn run() -> Result<u64, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    let mut count = 0;
    for result in rdr.byte_records() {
        let record = result?;
        if &record[0] == b"us" && &record[3] == b"MA" {
            count += 1;
        }
    }
    Ok(count)
}
*/
/*
//tutorial-perf-alloc-03.rs
fn run() -> Result<u64, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut record = csv::StringRecord::new(); // ByteRecord::new()

    let mut count = 0;
    while rdr.read_record(&mut record)? { //read_byte_record(&mut record)?
        if &record[0] == "us" && &record[3] == "MA" {
            count += 1;
        }
    }
    Ok(count)
}
*/

/*
//tutorial-perf-serde-01.rs
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    country: String,
    city: String,
    accent_city: String,
    region: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    population: Option<u64>,
    latitude: f64,
    longitude: f64,
}

fn run() -> Result<u64, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    let mut count = 0;
    for result in rdr.deserialize() {
        let record: Record = result?;
        if record.country == "us" && record.region == "MA" {
            count += 1;
        }
    }
    Ok(count)
}
*/
/*
//tutorial-perf-serde-02.rs
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record<'a> {
    country: &'a str,
    city: &'a str,
    accent_city: &'a str,
    region: &'a str,
    #[serde(deserialize_with = "csv::invalid_option")]
    population: Option<u64>,
    latitude: f64,
    longitude: f64,
}

fn run() -> Result<u64, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut raw_record = csv::StringRecord::new();
    let headers = rdr.headers()?.clone();


    let mut count = 0;
    while rdr.read_record(&mut raw_record)? {
        let record: Record = raw_record.deserialize(Some(&headers))?;
        if record.country == "us" && record.region == "MA" {
            count += 1;
        }
    }
    Ok(count)
}
*/

/*
//tutorial-perf-serde-03.rs
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record<'a> {
    country: &'a [u8],
    city: &'a [u8],
    accent_city: &'a [u8],
    region: &'a [u8],
    #[serde(deserialize_with = "csv::invalid_option")]
    population: Option<u64>,
    latitude: f64,
    longitude: f64,
}

fn run() -> Result<u64, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut raw_record = csv::ByteRecord::new();
    let headers = rdr.byte_headers()?.clone();


    let mut count = 0;
    while rdr.read_byte_record(&mut raw_record)? {
        let record: Record = raw_record.deserialize(Some(&headers))?;
        if record.country == b"us" && record.region == b"MA" {
            count += 1;
        }
    }
    Ok(count)
}
*/

//tutorial-perf-core-01.rs
/*
 * 1.If you're in an environment where the standard library is not usable.
 * 2.If you wanted to build your own csv-like library, you could build it on top of csv-core.
 */
use std::io::Read;

use csv_core::{Reader, ReadFieldResult};

fn run(mut data: &[u8]) -> Option<u64> {
    let mut rdr = Reader::new();

    // Count the number of records in Massachusetts.
    let mut count = 0;
    // Indicates the current field index. Reset to 0 at start of each record.
    let mut fieldidx = 0;
    // True when the current record is in the United States.
    let mut inus = false;
    // Buffer for field data. Must be big enough to hold the largest field.
    let mut field = [0; 1024];
    loop {
        // Attempt to incrementally read the next CSV field.
        let (result, nread, nwrite) = rdr.read_field(data, &mut field);
        // nread is the number of bytes read from our input. We should never
        // pass those bytes to read_field again.
        data = &data[nread..];
        // nwrite is the number of bytes written to the output buffer `field`.
        // The contents of the buffer after this point is unspecified.
        let field = &field[..nwrite];

        match result {
            // We don't need to handle this case because we read all of the
            // data up front. If we were reading data incrementally, then this 
            // would be a signal to read more.
            ReadFieldResult::InputEmpty => {}
            // If we get this case, then we found a field that contains more 
            // than 1024 bytes. We keep this example simple and just fail.
            ReadFieldResult::OutputFull => {
                return None;
            }
            // This case happens when we've successfully read a field. If the 
            // field is the last field in a record, then `record_end` is true.
            ReadFieldResult::Field { record_end } => {
                if fieldidx == 0 && field == b"us" {
                    inus = true;
                } else if inus && fieldidx == 3 && field == b"MA" {
                    count += 1;
                }
                if record_end {
                    fieldidx = 0;
                    inus = false;
                } else {
                    fieldidx += 1;
                }
            }
            // This case happens when the CSV reader has successfully exhausted
            // all input.
            ReadFieldResult::End => {
                break;
            }
        }
    }
    Some(count)
}


fn main() {
    /*
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
    */
    /*
    //tutorial-perf-serde-01.rs
    match run() {
        Ok(count) => {
            println!("{}", count);
        }
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    }
    */
    //tutorial-perf-core-01.rs
    // Read the entire contents of stdin up front.
    let mut data = vec![];
    if let Err(err) = io::stdin().read_to_end(&mut data) {
        println!("{}", err);
        process::exit(1);
    }
    match run(&data) {
        None => {
            println!("error: could not count records, buffer too small");
            process::exit(1);
        }
        Some(count) => {
            println!("{}", count);
        }
    }
}

/*
fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        match result {
            Err(err) => return Err(From::from(err)),
            Ok(record) => {
                println!("{:?}", record);
            }
        }
    }
    Ok(())
}
*/
