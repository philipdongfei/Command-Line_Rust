use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn file_double<P: AsRef<Path>>(file_path: P) -> i32 {
    let mut file = File::open(file_path).unwrap(); // error 1
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap(); //error 2
    let n: i32 = contents.trim().parse().unwrap(); // error 3
    2 * n
}

