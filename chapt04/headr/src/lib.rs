use clap::{App, Arg};
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: i64,//usize,
    bytes: Option<i64>,//Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust head")
        // What goes here?
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("print the first K lines instead of the first 10;")
                .default_value("10")

        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .takes_value(true)
                .conflicts_with("lines")
                .help("print the first K bytes of each file."),

        )
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),

        )
        .get_matches();
    // TODO: str to int
    let lines = matches.value_of("lines")
        .map(parse_positive_int)
        .transpose() // Option::transpose will turn this into a Result<Option>
        .map_err(|e| format!("illegal line count -- {}", e))?;
    let bytes = matches.value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(), 
        bytes,
    })
}


fn parse_positive_int(val: &str) -> MyResult<i64> {
    match val.parse::<i64>() {
        Ok(n) if n != 0 => Ok(n),
        _ => {
            //TODO: get last char
            //let count = val.chars().count();
            //let mut chars = val.chars();
            let v: Vec<&str> = val.split(|c| c == 'k' || c == 'K').collect();
            let first = v[0];
            match first.parse::<i64>() {
                Ok(mut n) if n != 0 => {
                    n *= 1024;
                    Ok(n)
                }
                _ => Err(From::from(val)),//Err(val.into()) or Err(Into::into(val)),
            }
        }

    }
}

#[test]
fn test_parse_positive_int() {
    // 3 is an OK integer
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);


    // Any string is an error
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // A zero is an error
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());

}

#[test]
fn test_parse_int() {
    // 3 is an OK integer
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("-3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), -3);

    let res = parse_positive_int("3K");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3*1024);

    let res = parse_positive_int("10K");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 10*1024);

    // Any string is an error
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // A zero is an error
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());

}


fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();

    for (file_num, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        &filename
                    );

                }

                if let Some(mut num_bytes) = config.bytes {
                    // read num bytes
                    let filelen = fs::metadata(&filename)?.len() as i64;
                    if num_bytes < 0 {
                        let t_bytes = filelen + num_bytes; 
                        if t_bytes <= 0  {
                            break;
                        } else {
                            num_bytes = t_bytes;
                        }
                    } 
                    let mut buffer = vec![0; num_bytes as usize];
                    // Use take to read the requested number of bytes
                    let mut handle = file.take(num_bytes as u64);
                    let bytes_read = handle.read(&mut buffer)?;
                    print!("{}",
                        String::from_utf8_lossy(&buffer[..bytes_read])//Convert the selected bytes into a string, which may not be valid UTF-8.
                    );

                } else {
                    // read n lines
                    let mut line = String::new();
                    let readlines = file.lines();
                    let totalines = readlines.count() as i64;
                    let mut endline = config.lines;
                    if endline < 0 {
                        endline = totalines + endline; 
                    }
                    let mut readfile = open(&filename).unwrap();
                    for _ in 0..endline {
                        let bytes = readfile.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    } 
                } 
                
            }
        }
    }
    Ok(())
}
