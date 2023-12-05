use adventofcode::*;
fn main() {
    let input = read_lines("input.txt");
    let result: i64 = input
        .iter()
        .map(|line| {
            let digits = line.chars().filter(|c| c.is_digit(10)).collect::<String>();
            let first = digits.chars().next().unwrap();
            let last = digits.chars().last().unwrap();
            [first, last]
                .iter()
                .collect::<String>()
                .parse::<i64>()
                .unwrap()
        })
        .sum();
    println!("{}", result);
}
