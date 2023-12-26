use std::error::Error;
use std::fs;
use std::io;
use std::num;

fn main() {
    // We have to jump through some hoops to actually get error values.
    let io_err: io::Error = io::Error::last_os_error();
    let parse_err: num::ParseIntError = "not a number".parse::<i32>().unwrap_err();

    // Ok, here are the conversions.
    let err1: Box<dyn Error> = From::from(io_err);
    let err2: Box<dyn Error> = From::from(parse_err);

    println!("err1: {}", err1);
    println!("err2: {}", err2);
}
