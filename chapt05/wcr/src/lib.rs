use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}


pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust wc")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input file(s)")
                .default_value("-")
                .multiple(true),
        )
        .arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .help("Show word count")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .help("Show byte count")
                .takes_value(false),
                
        )
        .arg(
            Arg::with_name("chars")
                .short("m")
                .long("chars")
                .help("Show char count")
                .takes_value(false)
                .conflicts_with("bytes"),

        )
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .help("Show line count")
                .takes_value(false),
        )
        .get_matches();

    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");

    if [words, bytes, chars, lines].iter().all(|v| v == &false) {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(), 
        lines, 
        words,
        bytes,
        chars,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}


pub fn run(config: Config) -> MyResult<()> {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;
    
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                match count(file) {
                    Err(err) => eprintln!("{}: {}", filename, err),
                    Ok(fileinfo) => {
                        print_fields(&config, &fileinfo, &filename);
                        total_lines += fileinfo.num_lines;
                        total_words += fileinfo.num_words;
                        total_bytes += fileinfo.num_bytes;
                        total_chars += fileinfo.num_chars;
                    }
                } 
            }
        }
    }

    if config.files.len() > 1 {
        let total_fileinfo = FileInfo {
            num_lines: total_lines,
            num_words: total_words,
            num_bytes: total_bytes,
            num_chars: total_chars,

        };
        let totalname = "total";
        print_fields(&config, &total_fileinfo, &totalname);

    }


    Ok(())
}

pub fn print_fields(config: &Config, fileinfo: &FileInfo, filename: &str) {
    if config.lines {
        print!("{:>8}", fileinfo.num_lines);
    
    }
    if config.words {
        print!("{:>8}", fileinfo.num_words);
    
    }
    if config.bytes {
        print!("{:>8}", fileinfo.num_bytes);
    }
    if config.chars {
        print!("{:>8}", fileinfo.num_chars);
    }
    if filename != "-" {
        println!(" {}", filename);
    } else {
        println!();
    }    

}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut buffer = String::new();

    loop {
        let line_bytes = file.read_line(&mut buffer).unwrap();
        if line_bytes == 0 {
            break;
        }
        num_bytes += line_bytes;
        num_lines += 1;
        num_words += buffer.split_whitespace().count();
        num_chars += buffer.chars().count();
        buffer.clear(); 
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}
