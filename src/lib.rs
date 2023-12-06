pub use itertools::Itertools;
pub use std::collections::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
pub use tracing::debug;
pub use tracing::error;
pub use tracing::info;
pub use tracing::trace;
pub use tracing::warn;
pub use tracing::*;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
pub use unsafed::*;
pub fn tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_file(true)
                .with_line_number(true)
                .without_time(),
        )
        .init();
}
#[track_caller]
pub fn read_lines(path: impl AsRef<Path>) -> Vec<String> {
    let path = path.as_ref();
    let file = File::open(path).unwrap();
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.unwrap()).collect()
}
#[track_caller]
pub fn read(path: impl AsRef<Path>) -> String {
    let path = path.as_ref();
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf
}
