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
    let map = vec.cmap(|x| x.cs());
    let m: usize = max([]);
    info!(?set_a);
}
