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
pub trait ExtraIntoIterator: IntoIterator {
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
    fn cmap<T, F>(&self, f: F) -> Vec<T>
    where
        F: FnMut(Self::Item) -> T,
        Self: Clone,
    {
        self.ii().map(f).collect()
    }
    fn sums(&self) -> String
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
impl<T: IntoIterator> ExtraIntoIterator for T {}
pub trait ExtraIterator: Iterator {
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
impl<T: Iterator> ExtraIterator for T {}
pub trait ExtraString: ToString {
    fn cs(&self) -> String {
        self.to_string()
    }
    fn ci<T: std::str::FromStr>(&self) -> T
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.to_string().parse::<T>().unwrap()
    }
}
impl<T: ToString> ExtraString for T {}
pub fn int<T: std::str::FromStr>(s: &str) -> T
where
    T::Err: std::fmt::Debug,
{
    s.parse::<T>().unwrap()
}
pub fn str(s: impl ToString) -> String {
    s.to_string()
}
pub fn slice<T>(a: &[T], start: Option<usize>, stop: Option<usize>, step: usize) -> Vec<&T> {
    let start = match start {
        Some(start) => start,
        None => 0,
    };
    let stop = match stop {
        Some(stop) => stop,
        None => a.len(),
    };
    a[start..stop].iter().step_by(step).collect()
}
pub fn len<T>(a: &[T]) -> usize {
    a.len()
}
pub fn max<T: Ord>(a: impl IntoIterator<Item = T>) -> T {
    a.into_iter().max().unwrap()
}
pub fn min<T: Ord>(a: impl IntoIterator<Item = T>) -> T {
    a.into_iter().min().unwrap()
}
pub fn set<T: std::hash::Hash + Eq>(a: impl IntoIterator<Item = T>) -> HashSet<T> {
    a.into_iter().collect()
}
