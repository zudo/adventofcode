use adventofcode::*;
fn main() {
    let input = read("input.txt");
    let mut sum = 0;
    for line in input {
        let mut a = line.split(":");
        let game = a.next().unwrap();
        let rounds = a.next().unwrap().split(";");
        let mut possible = true;
        for round in rounds {
            for color in round.split(",") {
                println!("{}", color);
                let n = color
                    .trim()
                    .split(" ")
                    .next()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
                if color.contains("red") && n > 12 {
                    possible = false;
                } else if color.contains("green") && n > 13 {
                    possible = false;
                } else if color.contains("blue") && n > 14 {
                    possible = false;
                }
            }
            // println!("{} {} {}", i, line, possible);
        }
        if possible {
            let a = game["Game ".len()..].parse::<i32>().unwrap();
            sum += a;
        }
    }
    println!("{}", sum);
}
