fn main() {
    if let Err(e) = sortr::get_args().and_then(sortr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
