use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    cmp::Ordering::*,

};
use crate::Column::*;


type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    file1: String,
    file2: String,
    show_col1: bool,
    show_col2: bool,
    show_col3: bool,
    insensitive: bool,
    delimiter: String,
}

enum Column<'a> {
    Col1(&'a str),
    Col2(&'a str),
    Col3(&'a str),
}


pub fn get_args() -> MyResult<Config> {
    let matches = App::new("commr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust comm")
        .arg(//file1
            Arg::with_name("file1")
                .value_name("FILE1")
                .help("Input file 1")
                .takes_value(true)
                .required(true),

        )
        .arg(//file2
            Arg::with_name("file2")
                .value_name("FILE2")
                .help("Input file 2")
                .takes_value(true)
                .required(true),

        )
        .arg(//col1
            Arg::with_name("suppress_col1")
                .short("1")
                .help("suppress column 1 (lines unique to FILE1)")
                .takes_value(false),

        )
        .arg(//col2
            Arg::with_name("suppress_col2")
                .short("2")
                .help("suppress column 2 (lines unique to FILE2)")
                .takes_value(false),

        )
        .arg(//col3
            Arg::with_name("suppress_col3")
                .short("3")
                .help("suppress column 3 (lines that appear in both files)")
                .takes_value(false),

        )
        .arg(//insensitive
            Arg::with_name("insensitive")
                .short("i")
                .help("Case-insensitive")
                .takes_value(false),

        )
        .arg(//delimiter
            Arg::with_name("delimiter")
                .short("d")
                .long("output-delimiter")
                .value_name("DELIM")
                .help("Output delimiter")
                .default_value("\t")
                .takes_value(true),

        )
        .get_matches();

    let file1 = matches.value_of("file1").unwrap().to_string();
    let file2 = matches.value_of("file2").unwrap().to_string();
    let insensitive = matches.is_present("insensitive");
    let show_col1 = !matches.is_present("suppress_col1");
    let show_col2 = !matches.is_present("suppress_col2");
    let show_col3 = !matches.is_present("suppress_col3");
    let delimiter = matches.value_of("delimiter").unwrap().to_string();

    Ok(Config {
        file1,
        file2,
        show_col1,
        show_col2,
        show_col3,
        insensitive,
        delimiter,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(
                    File::open(filename)
                        .map_err(|e| format!("{}: {}", filename, e))?,
        ))),
    }
}


pub fn run(config: Config) -> MyResult<()> {
    let print = |col: Column| {
        let mut columns = vec![];
        match col {
            Col1(val) => {
                if config.show_col1 {
                    columns.push(val);
                }
            }
            Col2(val) => {
                if config.show_col2 {
                    if config.show_col1 {
                        columns.push("");
                    }
                    columns.push(val);
                }
            }
            Col3(val) => {
                if config.show_col3 {
                    if config.show_col1 {
                        columns.push("");
                    }
                    if config.show_col2 {
                        columns.push("");
                    }
                    columns.push(val);
                }
            }
        };
    
        if !columns.is_empty() {
            println!("{}", columns.join(&config.delimiter));
        }
    };

    let file1 = &config.file1;
    let file2 = &config.file2;

    if file1 == "-" && file2 == "-" {
        return Err(From::from("Both input files cannot be STDIN (\"-\")"));
    }

    let case = |line: String| {
        if config.insensitive {
            line.to_lowercase()
        } else {
            line
        }
    };
    let mut lines1 = open(file1)?.lines().filter_map(Result::ok).map(case);
    let mut lines2 = open(file2)?.lines().filter_map(Result::ok).map(case);

    let mut line1 = lines1.next();
    let mut line2 = lines2.next();
    
    while line1.is_some() || line2.is_some() {
        match (&line1, &line2) {
            (Some(val1), Some(val2)) => match val1.cmp(val2) {
                Equal => {

                    print(Col3(val1));
                    line1 = lines1.next();
                    line2 = lines2.next();
                }
                Less => {
                    print(Col1(val1));
                    line1 = lines1.next();

                }
                Greater => {
                    print(Col2(val2));
                    line2 = lines2.next();
                }
            },
            (Some(val1), None) => {
                print(Col1(val1));
                line1 = lines1.next();
            }
            (None, Some(val2)) => {
                print(Col2(val2));
                line2 = lines2.next();
            }
            _ => (),
        }
    }
    /*
    let mut file1_h = open(file1)?;
    let mut file2_h = open(file2)?;
    //println!("Opened {} and {}",file1, file2);
    let mut col1 = vec![];
    let mut col2 = vec![];
    let mut line = String::new();

    loop {
        let bytes = file1_h.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        col1.push(mem::take(&mut line));
        line.clear();
    };
    
    col1.sort();

    loop {
        let bytes = file2_h.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        
        col2.push(mem::take(&mut line));
        line.clear();
    };
    col2.sort();

    let max_len = col1.len() + col2.len();
    if max_len > col1.len() {
        col1.resize(max_len, "".to_string());
    }
    if max_len > col2.len() {
        col2.resize(max_len, "".to_string());
    }
    let mut col3 = Vec::with_capacity(max_len);
    col3.resize(max_len, "");
    println!("file1 {} lines {}", &file1, &col1.len());
    for col in col1 {
        print!("{}", &col);
    } 
    println!("file2 {} lines {}", &file2, &col2.len());
    for col in col2 {
        print!("{}", &col);
    } 
    println!("col3 len {}", &col3.len());
    for col in col3 {
        print!("{}", &col);
    }
    */
    Ok(())
}
