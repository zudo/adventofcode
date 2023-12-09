use adventofcode::*;
fn main() {
    tracing();
    let input = read("input.txt");
    let lines = input.trim().csplit("\n");
    let mut out = vec![];
    for line in lines {
        let mut tree = vec![];
        let mut previous = line.csplit(" ").mint::<i128>();
        for _ in 0..previous.len() {
            if !previous.cii().any(|x| x != 0) {
                break;
            }
            tree.push(previous.cv());
            let mut next = vec![];
            for w in previous.windows(2) {
                let a = w[1] - w[0];
                info!(?a);
                next.push(a);
            }
            previous = next;
            // info!(?previous);
        }
        info!(?tree);
        tree.reverse();
        for i in 0..tree.len() - 1 {
            let a = tree[i].cv().last().cloned().unwrap_or(0);
            let b = tree[i + 1].cv().last().cloned().unwrap_or(0);
            let leaf = a + b;
            info!(?a, ?b, ?leaf);
            tree[i + 1].push(leaf);
        }
        out.push(tree.last().unwrap().last().cloned().unwrap());
        info!(?tree);
        // for number in numbers {
        //     if b.last().unwrap_or(&0) < &number {
        //         b.push(number);
        //     }
        // }
    }
    let sum = sum(out);
    info!(?sum);
}
