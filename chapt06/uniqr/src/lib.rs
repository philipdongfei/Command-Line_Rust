use clap::{App, Arg};
use std::{
    error::Error,
    fs::{File },
    io::{self, BufRead, BufReader, Write },
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String, 
    out_file: Option<String>,
    count: bool,
}


pub fn get_args() -> MyResult<Config> {
    let matches = App::new("uniqr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust uniq")
        .arg (
            Arg::with_name("count")
                .short("c")
                .long("count")
                .help("Show counts")
                .takes_value(false),
        )
        .arg (
            Arg::with_name("in_file")
                .value_name("IN_FILE")
                .help("Input file [default: -]")
                .default_value("-"),

        )
        .arg (
            Arg::with_name("out_file")
                .value_name("OUT_FILE")
                .help("Output file"),
        )
        .get_matches();

    let infile = matches.value_of_lossy("in_file").unwrap().to_string(); // | matches.value_of_lossy("in_file").map(String::from).unwrap(); 
    // | matches.value_of_lossy("in_file").map(|v| v.into()).unwrap();
    // | matches.value_of_lossy("in_file").map(Into::into).unwrap();
    let outfile = matches.value_of("out_file").map(String::from); // | matches.value_of("out_file").map(|v| v.to_string());
    let count = matches.is_present("count");

    Ok(Config {
        in_file: infile, 
        out_file: outfile, //outfile.map(|s| s.to_string()),
        count,
    })
}

pub fn my_run(config: Config) -> MyResult<()> {
    let mut out_file: Box<dyn Write> = Box::new(io::stdout());
    if let Some(ref path)= config.out_file {
        if !path.is_empty()  {
            out_file = Box::new(File::create(path)?);
            
        } 
    }
    
    let mut print = |count: u64, text: &str| -> MyResult<()> {
        if count > 0 {
            let line ;//= String::new();
            if config.count {
                line = format!("{:>4} {}", count, text);
            } else {
                line = format!("{}", text);
            }  

            out_file.write(line.as_bytes())?;
        }
        Ok(())
    };

    let mut file = open(&config.in_file)
        .map_err(|e| format!("{}: {}", config.in_file, e))?;
    let mut line = String::new();
    let mut previous = String::new();
    //let mut vec_lines: Vec<UniqLine> = Vec::new();
    let mut count = 0;
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        if previous.is_empty() || !previous.trim_end().eq(line.trim_end()){
            if count > 0 {
                print(count, &previous)?;
                previous = line.clone();
                count = 0;
            } 
            
        } 
        count += 1;
        line.clear();
    }
    print(count, &previous)?;

    Ok(())
}

pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file)
        .map_err(|e| format!("{}: {}", config.in_file, e))?;

    let mut out_file: Box<dyn Write> = match &config.out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };

    let mut print = |count: u64, text: &str| -> MyResult<()> {
        if count > 0 {
            if config.count {
                write!(out_file, "{:>4} {}", count, text)?;
            } else {
                write!(out_file, "{}", text)?;
            }
        };
        Ok(())
    };

    let mut line = String::new();
    let mut previous = String::new();
    let mut count: u64 = 0;

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if line.trim_end() != previous.trim_end() {
            print(count, &previous)?;
            previous = line.clone();
            count = 0;
        }

        count += 1;
        line.clear();
    }

    print(count, &previous)?;
    Ok(())
}


fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

