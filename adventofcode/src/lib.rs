use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
pub fn read(path: impl AsRef<Path>) -> Vec<String> {
    let path = path.as_ref();
    let file = File::open(path).unwrap();
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.unwrap()).collect()
}
