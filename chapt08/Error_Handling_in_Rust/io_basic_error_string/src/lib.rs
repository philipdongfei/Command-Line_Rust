use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    File::open(file_path)
        .map_err(|err| err.to_string())
        .and_then(|mut file| {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|err| err.to_string())
                .map(|_| contents)
        })
    .and_then(|contents| {
        contents.trim().parse::<i32>()
                .map_err(|err| err.to_string())
    })
    .map(|n| 2 * n)
}

