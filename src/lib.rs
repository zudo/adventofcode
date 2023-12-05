use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
pub mod tracing;
pub use tracing::*;
pub fn read_lines(path: impl AsRef<Path>) -> Vec<String> {
    let path = path.as_ref();
    let file = File::open(path).unwrap();
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.unwrap()).collect()
}
pub fn read(path: impl AsRef<Path>) -> String {
    let path = path.as_ref();
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf
}
