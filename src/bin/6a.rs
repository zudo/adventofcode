use adventofcode::*;
fn main() {
    tracing();
    let text = read("input.txt");
    let vec = text.split("\n").cv();
    let set_a = vec.cset();
    let set_a = set(&vec);
    let i: usize = int("123");
    let s = str("asdf");
    let ii = vec.ii();
    let ct: (&str, &str) = ii.ct().unwrap();
    let ct2: (&str, &str) = ii.ct().unwrap();
    let map = vec.cmap(|x| x.cs());
    let ints: Vec<u32> = vec.cmap(|x| x.ci());
    let a = max(ints);
    info!(?set_a);
}
