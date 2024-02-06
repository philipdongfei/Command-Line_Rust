use clap::{App, Arg};
use std::error::Error;
use chrono::{NaiveDate, Datelike, Local};


#[derive(Debug)]
pub struct Config {
    month: Option<u32>,
    year: i32,
    today: NaiveDate,
}

type MyResult<T> = Result<T, Box<dyn Error>>;


pub fn get_args() -> MyResult<Config> {
    let matches = App::new("calr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust cal")
        /*
        .arg(
            // flags
        )
        .arg(
            // month
        )
        .arg(
            // year
        )
        */
        .get_matches();

    let today = Local::today();
    Ok(Config {
        month: Some(today.month()),
        year: today.year(),
        today: today.naive_local(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}
