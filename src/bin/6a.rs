use adventofcode::*;
fn main() {
    tracing();
    let text = read("input.txt");
    let vec = text.split("\n").cv();
    let set = vec.cset();
    info!(?set);
}
