use adventofcode::*;
fn main() {
    tracing();
    let text = read("input.txt");
    let vec = text.split("\n").cv();
    let set = vec.cset();
    let ii = vec.ii();
    let ct: (&str, &str) = ii.ct().unwrap();
    let ct2: (&str, &str) = ii.ct().unwrap();
    info!(?set);
}
