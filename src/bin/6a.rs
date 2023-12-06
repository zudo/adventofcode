use adventofcode::*;
fn main() {
    tracing();
    let input = read("input.txt");
    let mut a = input.split("\n");
    let b = a.next().unwrap().split(":").nth(1).unwrap().trim();
    let c = a.next().unwrap().split(":").nth(1).unwrap().trim();
    let times = b.split(" ").filter(|s| !s.is_empty()).map(int::<i64>).ii();
    let distances = c.split(" ").filter(|s| !s.is_empty()).map(int::<i64>).ii();
    let races = times.zip(distances).cv();
    info!(?races);
    let mut wins = 1;
    for race in races {
        let mut a = 0;
        for i in 0..race.0 {
            if i * (race.0 - i) > race.1 {
                a += 1;
            }
        }
        wins *= a;
    }
    info!(?wins);
}
