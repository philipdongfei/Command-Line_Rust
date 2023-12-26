fn main() {
    match io_basic_error_custom_from::file_double_verbose("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {:?}", err),
    }
}
