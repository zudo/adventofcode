use adventofcode::*;
const CARDS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];
fn main() {
    tracing();
    let input = read("input.txt")
        .replace("T", char::from(101).to_string().as_str())
        .replace("J", char::from(102).to_string().as_str())
        .replace("Q", char::from(103).to_string().as_str())
        .replace("K", char::from(104).to_string().as_str())
        .replace("A", char::from(105).to_string().as_str());
    let vec = input.trim().csplit("\n");
    info!(?vec);
    let mut hands = vec
        .ii()
        .map(|x| x.csplit(" ").ca::<2>())
        .collect::<Vec<_>>();
    info!(?vec);
    hands.sort_by(|a, b| {
        let o = get_value_of_hand(&a[0]).cmp(&get_value_of_hand(&b[0]));
        if o == std::cmp::Ordering::Equal {
            let c = a[0].cmp(&b[0]);
            info!(?a, ?b, ?c);
            c
        } else {
            o
        }
    });
    let mut sum: u128 = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += (i + 1) as u128 * hand[1].ci::<u128>();
    }
    info!(?sum);
}
fn get_value_of_hand(cards: &str) -> usize {
    let mut map = HashMap::new();
    for c in cards.chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    let vec = map.values().cv();
    // info!(?vec);
    let m = *max(vec.cv());
    if m == 5 {
        return 10000;
    }
    if m == 4 {
        return 8000;
    }
    if vec.len() == 2 {
        return 5000;
    }
    if m == 3 {
        return 4000;
    }
    if vec.len() == 3 {
        // info!(?cards);
        return 2000;
    }
    if vec.len() == 4 {
        info!(?cards);
        return 1000;
    }
    return 0;
    let vec = cards.chars().map(|x| x as u8).cv();
    let m = max(vec.cv());
    // info!(?vec, ?m);
    return m as usize;
}
