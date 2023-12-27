// These use statements were added below the `extern` statements.
// I'll elide them in the future. Don't worry! It's all on Github:
// https://github.com/BurntSushi/rust-error-handling-case-study
use std::io::{self, Write};
use std::process;
use std::{
    error::Error,
    env,
    fs,
};
use docopt::Docopt;
use serde::Deserialize;
use serde::Deserializer;
//extern crate docopt;
//extern crate rustc_serialize;

static USAGE: &'static str = "
Usage: city-pop [options] [<data-path>] <city>
       city-pop --help

Options:
    -h, --help      Show this usage message.
    -q, --quiet     Don't show noisy messages.
";

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct Args {
    arg_data_path: Option<String>,
    arg_city: String,
    flag_quiet: bool,
}

/*
// This struct represents the data in each row of the CSV file.
// Type based decoding absolves us of a lot of the nitty gritty error
// handling, like parsing strings as integers or floats.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Row {
    country: String,
    city: String,
    accent_city: String,
    region: String,

    // Not every row has data for the population, latitude or longitude!
    // So we express them as `Option` types, which admits the possibility of
    // absence. The CSV parse will fill in the correct value for us.
    //#[serde(deserialize_with = "deserialize_null_default")]
    //#[serde(skip_serializing_if = "Option::is_none")]
    population: Option<u64>,
    latitude: Option<f64>,
    longitude: Option<f64>,
}
*/

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

struct PopulationCount {
    city: String,
    country: String,
    // This is no longer an `Option` because values of this type are only
    // constructed if they have a population count.
    count: u64,
}

/*
enum CliError {
    Io(io::Error),
    Csv(csv::Error),
    NotFound,
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut self.fmt::Formatter) -> self.fmt::Result {
        match *self {
            CliError::Io(ref err) => err.fmt(f),
            CliError::Csv(ref err) => err.fmt(f),
            CliError::NotFound => write!(f, "No matching client a \
                                        population were found."),
        }
    }
}

impl Error for CliError {
    fn description(&self) -> &str {
        match *self {
            CliError::Io(ref err) => err.description(),
            CliError::Csv(ref err) => err.description(),
            CliError::NotFound => "not found",
        }
    }
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}


impl From<csv::Error> for CliError {
    fn from(err: csv::Error) -> CliError {
        CliError::Csv(err)
    }
}
*/


fn search<P: AsRef<std::path::Path> + std::convert::AsRef<std::path::Path>>(file_path: &Option<P>, city: &str) -> Result<Vec<PopulationCount>,/*CliError*/Box<dyn Error+Send+Sync>> {
    let mut found = vec![];
    let input: Box<dyn io::Read> = match *file_path {
        None => Box::new(io::stdin()),
        Some(ref file_path) => Box::new(fs::File::open(file_path)?),
    };
    let mut rdr = csv::Reader::from_reader(input);
    for row in rdr.deserialize() {
        let row: Row = row?;
        match row.population {
            None => {} // skip it
            Some(count) => if row.city == city {
                found.push(PopulationCount {
                    city: row.city,
                    country: row.country,
                    count: count,
                });
            },
        }
    }
    if found.is_empty() {
        //Err(CliError::NotFound)
        Err(From::from("No matching cities with a population were found."))
    } else {
        Ok(found)
    }
}


/*
fn deserialize_null_default<'de,T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where 
    D: Deserializer<'de>,
    T: Deserialize<'de>,

{
    Ok(Option::deserialize(deserializer)?)
}
*/

/*
fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error> 
where 
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>, 
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
*/


fn main() {
    /*
    let args: Args = match Docopt::new(USAGE) {
        Err(err) => {
            writeln!(&mut io::stderr(), "{}", err).unwrap();
            process::exit(1);
        }
        Ok(dopt) => match dopt.argv(env::args().into_iter()).deserialize() 
        {
            Err(err) => {
                writeln!(&mut io::stderr(), "{}", err).unwrap();
                process::exit(1);
            }
            Ok(args) => args, 
        }
    };
    */
    let args: Args = Docopt::new(USAGE)
        .and_then(|dopt| dopt.argv(env::args().into_iter()).deserialize())
        .unwrap_or_else(|err| err.exit());
    /*
    match search(&args.arg_data_path, &args.arg_city) {
        Err(CliError::NotFound) if args.flag_quiet => process::exit(1),
        Err(err) => fatal!("{}", err),
        Ok(pops) => for pop in pops {
            println!("{}, {}: {:?}", pop.city, pop.country, pop.count);
        }
    }
    */
    println!("{:?},{}", &args.arg_data_path, &args.arg_city);
    for pop in search(&args.arg_data_path, &args.arg_city).unwrap() {
        println!("{}, {}: {:?}", pop.city, pop.country, pop.count);
    }
    /*
    let file = fs::File::open(args.arg_data_path).unwrap();
    let mut rdr = csv::Reader::from_reader(file);
    let mut iter = rdr.deserialize();

    if let Some(result) = iter.next() {
        let record: Row = result.unwrap();
        println!("{:?}", record);
    } else {
        //Err(From::from("expected at least one record but get none"));
        process::exit(1);
    }
    */
    /*
    for Some(iter) in rdr.deserialize() {
        let row: Row = iter.unwrap();
        //println!("{:?}", row);
        if row.city == args.arg_city {
            println!("{}, {}: {:?}",
                row.city, row.country,
                row.population.expect("population count"));
        }
    }
    */

}

