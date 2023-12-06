use adventofcode::*;
fn main() {
    tracing();
    let input = read("input.txt");
    let mut a = input.split("\n");
    let time = a
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .replace(" ", "")
        .cs();
    let distance = a
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .replace(" ", "")
        .cs();
    info!(?time, ?distance);
    let time: i64 = int(&time);
    let distance: i64 = int(&distance);
    info!(?time, ?distance);
    let mut wins = 0;
    for i in 1..time {
        if i * (time - i) > distance {
            wins += 1;
        }
    }
    info!(?wins);
}
