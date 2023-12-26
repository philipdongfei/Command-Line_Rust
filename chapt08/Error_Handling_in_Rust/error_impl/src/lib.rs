use std::io;
use std::num;

#[derive(Debug)]
pub enum CliError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

use std::error;
use std::fmt;

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to 
            // their implementations.
            CliError::Io(ref err) => write!(f, "IO error: {}", err),
            CliError::Parse(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl error:: Error for CliError {
    fn description(&self) -> &str {
        // Both underlying errors already impl `Error`, so we defer to their
        // implementations.
        match *self {
            CliError::Io(ref err) => err.description(),
            // Normally we can just write `err.description()`, but the error
            // type has a concrete method called `description`, which confilicts
            // with the trait method. For now, we must explicitly call
            // `description` through the `Error` trait.
            CliError::Parse(ref err) => error::Error::description(err),
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            // N.B. Both of these implicitly case `err` from their concrete
            // types (either `&io::Error` or `&num::ParseIntError`)
            // to a trait object `&Error`. This works because both error types
            // implement `Error`.
            CliError::Io(ref err) => Some(err),
            CliError::Parse(ref err) => Some(err),
        }
    }
}

