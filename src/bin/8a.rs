use adventofcode::*;
fn main() {
    tracing();
    let input = read("input.txt");
    let vec = input.trim().csplit("\n");
    let instructions = vec.first().unwrap().trim();
    info!(?instructions);
    let mut map = HashMap::new();
    for line in vec.ii().skip(2) {
        let k = line[0..3].to_string();
        let v_l = line[7..10].to_string();
        let v_r = line[12..15].to_string();
        info!(?k, ?v_l, ?v_r);
        map.insert(k, (v_l, v_r));
    }
    let mut i: usize = 0;
    let mut k = "AAA".to_string();
    loop {
        let c = instructions.chars().nth(i % instructions.len()).unwrap();
        i += 1;
        if c == 'L' {
            k = map.get(&k).unwrap().0.to_string();
        } else {
            k = map.get(&k).unwrap().1.to_string();
        }
        if k == "ZZZ" {
            break;
        }
    }
    info!("{}", i);
}
