use std::env;

fn main() {
    match error_double_string::double_arg(env::args()) {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }

}
