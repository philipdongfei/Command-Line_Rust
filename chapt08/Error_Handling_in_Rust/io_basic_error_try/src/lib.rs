use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    let mut file = r#try!(File::open(file_path).map_err(|e| e.to_string()));
    let mut contents = String::new();
    r#try!(file.read_to_string(&mut contents).map_err(|e| e.to_string()));
    let n = r#try!(contents.trim().parse::<i32>().map_err(|e| e.to_string()));
    Ok(2 * n)
}
