fn main() {
    match io_basic_error_string_early_return::file_double("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
}
