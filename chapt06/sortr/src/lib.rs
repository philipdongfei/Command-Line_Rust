use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},

};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,//default sort dictionary order
    //dict: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("sortr")
        .version("0.1.0")
        .author("Philip Tung <Philip.Tungfei@gmail.com>")
        .about("Rust sort")
        .arg(
            Arg::with_name("in_file")
                .value_name("IN_FILE")
                .help("Input file")
                .default_value("-"),

        )
        .get_matches();

    let in_file = matches.value_of_lossy("in_file").unwrap().to_string(); // | matches.value_of_lossy("in_file").map(String::from).unwrap(); 

    Ok(Config {
        in_file,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}



pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file)
        .map_err(|e| format!("{}: {}", config.in_file, e))?;

    let mut line = String::new();
    let mut lines: Vec<String> = Vec::new();
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        //print!("{}", line);
        lines.push(line.clone().trim_end().to_string());
        line.clear();
    }

    if !lines.is_empty() {
        lines.sort();

        for l in lines.iter() {
            print!("{}\n", l);
        } 

    }
    Ok(())
}
