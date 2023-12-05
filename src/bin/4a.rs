use adventofcode::*;
fn main() {
    let lines = read_lines("input.txt");
    let mut sum = 0;
    for (row, line) in lines.iter().enumerate() {
        let a = line.split(":").collect::<Vec<_>>();
        let b = a[1].split("|").collect::<Vec<_>>();
        let left = b[0];
        let right = b[1];
        println!("{}", right);
        let left_numbers = left
            .split(" ")
            .filter(|x| x != &"")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let right_numbers = right
            .split(" ")
            .filter(|x| x != &"")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        println!("{:?}", left_numbers);
        let mut mul = 1;
        for left_number in left_numbers {
            for right_number in right_numbers.clone() {
                if left_number == right_number {
                    mul *= 2;
                }
            }
        }
        println!("{}", mul);
        if mul == 1 {
            continue;
        }
        sum += mul / 2;
    }
    println!("{}", sum);
}
