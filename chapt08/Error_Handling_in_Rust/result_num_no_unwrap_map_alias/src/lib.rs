use std::num::ParseIntError;
use std::result;

type Result<T> = result::Result<T, ParseIntError>;

pub fn double_number(number_str: &str) -> Result<i32> {
    //unimplemented!();
    number_str.parse::<i32>().map(|n| 2 * n)

}
