use adventofcode::*;
fn main() {
    tracing();
    let lines = read("input.txt");
    info!(?lines);
    let cc = vec![1; lines.len()];
    for (row, line) in lines.iter().enumerate() {
        debug!(?row, ?line);
    }
    let sum = cc.iter().sum::<usize>();
    info!(?sum);
}
