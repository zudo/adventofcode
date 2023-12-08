use adventofcode::*;
fn main() {
    tracing();
    let input = read("input.txt");
    let vec = input.trim().csplit("\n");
    let instructions = vec.first().unwrap().trim();
    let mut map = HashMap::new();
    for line in vec.ii().skip(2) {
        let k = line[0..3].to_string();
        let v_l = line[7..10].to_string();
        let v_r = line[12..15].to_string();
        map.insert(k, (v_l, v_r));
    }
    let keys = map
        .keys()
        .cv()
        .ii()
        .filter(|k| k.ends_with("A"))
        .cmap(|k| k.to_string());
    info!(?keys);
    let mut out = vec![];
    for key in keys {
        let mut k = key.clone();
        let mut i = 0;
        loop {
            let c = instructions.chars().nth(i % instructions.len()).unwrap();
            i += 1;
            if c == 'L' {
                k = map.get(&k).unwrap().0.to_string();
            } else {
                k = map.get(&k).unwrap().1.to_string();
            }
            if k.ends_with("Z") {
                break;
            }
        }
        out.push(i as u128);
    }
    info!(?out);
    let mut a = 1;
    for &num in &out {
        let mut b = num;
        let mut c = a;
        while b != 0 {
            let d = b;
            b = c % b;
            c = d;
        }
        a = (a / c) * num;
    }
    info!(?a);
}
