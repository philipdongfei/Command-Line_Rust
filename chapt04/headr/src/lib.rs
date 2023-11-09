use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
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
                .long("K_lines")
                .value_name("LINES")
                .takes_value(true)
                .default_value("10")
                .conflicts_with("bytes")
                .help("print the first K lines instead of the first 10;"),

        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("K_bytes")
                .value_name("BYTES")
                .takes_value(true)
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
    let lines = matches.value_of("lines").parse()::<usize>.unwrap();
    let bytes = matches.value_of("bytes").parse()::<option<usize>>;

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: 
        bytes,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}
