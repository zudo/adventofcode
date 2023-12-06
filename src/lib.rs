pub use itertools::Itertools;
pub use std::collections::*;
use std::fmt::Display;
use std::fs::File;
use std::hash::Hash;
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
pub trait ExtraItertools: IntoIterator + Sized {
    fn cset(self) -> HashSet<Self::Item>
    where
        Self::Item: Eq + Hash,
    {
        self.ii().collect()
    }
    fn cs(self) -> String
    where
        Self::Item: Display,
    {
        self.ii().map(|x| x.to_string()).collect()
    }
    fn cii(&self) -> std::vec::IntoIter<Self::Item>
    where
        Self: Clone,
    {
        self.clone().into_iter().collect_vec().into_iter()
    }
    fn ii(self) -> Self::IntoIter {
        self.into_iter()
    }
    fn test(
        self,
        mut pass: impl FnMut(&Self::Item) -> bool,
        mut fail: impl FnMut(&Self::Item) -> bool,
    ) -> bool {
        for item in self {
            if pass(&item) {
                return true;
            }
            if fail(&item) {
                return false;
            }
        }
        unreachable!("the iterator does not pass or fail");
    }
    fn one(&self) -> Self::Item
    where
        Self: Clone,
    {
        let mut iter = self.cii();
        let item = iter.next().unwrap();
        assert!(iter.next().is_none());
        item
    }
    fn cv(self) -> Vec<Self::Item> {
        self.into_iter().collect()
    }
    fn ca<const N: usize>(self) -> [Self::Item; N]
    where
        <Self as std::iter::IntoIterator>::Item: std::fmt::Debug,
    {
        self.cv().try_into().unwrap()
    }
    fn sumi(self) -> Self::Item
    where
        <Self as std::iter::IntoIterator>::Item: std::ops::Add<Output = Self::Item> + Default,
    {
        self.ii().fold(Default::default(), |a, b| a + b)
    }
}
impl<T: IntoIterator + Sized + Clone> ExtraItertools for T {}
