use std::io;
use std::num;
use std::fs::File;
use std::io::Read;
use std::path::Path;

// We derive `Debug` because all types should probably derive `Debug`.
// This gives us a reasonable human readable description of `CliError` values.
#[derive(Debug)]
pub enum CliError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

pub fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
    let mut file = File::open(file_path).map_err(CliError::Io)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(CliError::Io)?;
    let n: i32 = contents.trim().parse().map_err(CliError::Parse)?;
    Ok(2 * n)
}
