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
pub trait ExtraIntoIterator: IntoIterator + Sized {
    fn ii(&self) -> Self::IntoIter
    where
        Self: Clone,
    {
        self.clone().into_iter()
    }
    fn cset(&self) -> HashSet<Self::Item>
    where
        Self::Item: Eq + std::hash::Hash,
        Self: Clone,
    {
        self.ii().collect()
    }
    fn cstring(&self) -> String
    where
        Self::Item: ToString,
        Self: Clone,
    {
        self.ii().map(|x| x.to_string()).collect()
    }
    fn sumi(&self) -> Self::Item
    where
        <Self as std::iter::IntoIterator>::Item: std::ops::Add<Output = Self::Item> + Default,
        Self: Clone,
    {
        self.ii().fold(Default::default(), |a, b| a + b)
    }
}
impl<T: IntoIterator + Sized + Clone> ExtraIntoIterator for T {}
pub trait ExtraIterator: Iterator + Sized {
    fn cv(&self) -> Vec<Self::Item>
    where
        Self: Clone,
    {
        self.clone().collect_vec()
    }
    fn ca<const N: usize>(&self) -> [Self::Item; N]
    where
        <Self as std::iter::IntoIterator>::Item: std::fmt::Debug,
        Self: Clone,
    {
        self.cv().try_into().unwrap()
    }
    fn ct<T>(&self) -> Option<T>
    where
        Self: Sized + Iterator<Item = T::Item>,
        T: itertools::traits::HomogeneousTuple,
        Self: Clone,
    {
        self.clone().collect_tuple()
    }
}
impl<T: Iterator + Sized + Clone> ExtraIterator for T {}
