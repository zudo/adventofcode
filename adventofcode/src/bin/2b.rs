use adventofcode::*;
fn main() {
    let input = read("input.txt");
    let mut sum = 0;
    for line in input {
        let mut a = line.split(":");
        let _ = a.next().unwrap();
        let rounds = a.next().unwrap().split(";");
        let possible = true;
        let mut blue = 0;
        let mut red = 0;
        let mut green = 0;
        for round in rounds {
            for color in round.split(",") {
                let n = color
                    .trim()
                    .split(" ")
                    .next()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
                println!("{}", color);
                // if color.contains("red") && n > 12 {
                // possible = false;
                // }
                // if color.contains("green") && n > 13 {
                // possible = false;
                // }
                // if color.contains("blue") && n > 14 {
                // possible = false;
                // }
                if possible {
                    if color.contains("red") && n > red {
                        red = n;
                    } else if color.contains("green") && n > green {
                        green = n;
                    } else if color.contains("blue") && n > blue {
                        blue = n;
                    }
                }
            }
            // println!("{} {} {}", i, line, possible);
            println!("{}, {}, {}", red, green, blue);
        }
        if possible {
            sum += blue * red * green;
        }
    }
    println!("{}", sum);
}
