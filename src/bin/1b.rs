use adventofcode::*;
fn to_digit(s: &str) -> Option<&str> {
    match s {
        "one" => Some("1"),
        "two" => Some("2"),
        "three" => Some("3"),
        "four" => Some("4"),
        "five" => Some("5"),
        "six" => Some("6"),
        "seven" => Some("7"),
        "eight" => Some("8"),
        "nine" => Some("9"),
        _ => None,
    }
}
fn main() {
    let input = read("input.txt");
    let digits = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let mut vec_a = vec![];
    for line in input {
        let mut vec_b = vec![];
        for i in 0..line.len() {
            let line = line.get(i..).unwrap();
            for digit in digits {
                if line.starts_with(digit) {
                    if vec_b.len() < 2 {
                        vec_b.push(digit);
                    } else {
                        vec_b[1] = digit;
                    }
                }
            }
        }
        if vec_b.len() != 2 {
            vec_b.push(vec_b[0]);
        }
        // println!("{:?}", vec_b);
        let mut vec_c = vec![];
        for digit in &vec_b {
            if let Some(digit) = to_digit(digit) {
                vec_c.push(digit);
            } else {
                vec_c.push(digit);
            }
        }
        // println!("{:?}", vec_c);
        let a = vec_c.join("").parse::<i64>().unwrap();
        vec_a.push(a);
    }
    println!("{}", vec_a.iter().sum::<i64>());
}
