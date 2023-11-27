use clap::{App, Arg};
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write },
    path::{Path},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String, 
    out_file: Option<String>,
    count: bool,
}

pub struct UniqLine {
    uline: String,
    nums: u32,
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

pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file)
        .map_err(|e| format!("{}: {}", config.in_file, e))?;
    let mut line = String::new();
    let mut vec_lines: Vec<UniqLine> = Vec::new();
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        if vec_lines.is_empty() || vec_lines.last().map(|l| l.uline.eq(&line)) == Some(false) {
            vec_lines.push(UniqLine {
                uline: line.clone(),
                nums: 1,
            });
        } else {
            vec_lines.last_mut().map(|l| l.nums += 1);
        } 
        line.clear();
    }
    line.clear(); 
    let mut out_file: Box<dyn Write> = Box::new(io::stdout());
    if let Some(ref path)= config.out_file {
        if !path.is_empty()  {
            out_file = Box::new(File::create(path)?);//OpenOptions::new().write(true).truncate(true).open(path)?;//File::create(path)?;
            //println!("write one");
            
        } 
        /*
        else {
            out_file = Box::new(io::stdout());
            //out_file.write(line.as_bytes())?;
        }
        */
    }

    for uniq_line in vec_lines.iter() {
        if config.count {
            line = format!("{:>4} {}", uniq_line.nums, uniq_line.uline);
        } else {
            line = format!("{}", uniq_line.uline);
        }  
        if config.out_file == None {
            print!("{}", line);
        } else {
            //let mut out_file: Box<dyn Write>;
            out_file.write(line.as_bytes())?;
            line.clear();
        }
    } 
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

