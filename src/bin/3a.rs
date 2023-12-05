use adventofcode::*;
fn main() {
    let input = read_lines("input.txt");
    let mut sum = 0;
    for (i, line) in input.iter().enumerate() {
        let mut n = 0;
        let mut adjacent = false;
        for (j, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                n = format!("{}{}", n, c).parse::<u64>().unwrap();
                for k in i.saturating_sub(1)..=i + 1 {
                    if let Some(line) = input.get(k) {
                        for l in j.saturating_sub(1)..=j + 1 {
                            if let Some(c) = line.chars().nth(l) {
                                // print!("{}", c);
                                if c != '.' && !c.is_digit(10) {
                                    adjacent = true;
                                }
                            }
                        }
                    }
                    // println!()
                }
                if j == line.len() - 1 {
                    if adjacent {
                        sum += n;
                        // println!("{}", n);
                    }
                }
                // println!("{}", adjacent);
            } else {
                if adjacent {
                    sum += n;
                    // println!("{}", n);
                }
                n = 0;
                adjacent = false;
            }
        }
    }
    println!("{}", sum);
}
