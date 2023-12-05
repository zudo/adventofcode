use adventofcode::*;
fn main() {
    let lines = read_lines("input.txt");
    let mut cc = vec![1; lines.len()];
    for (row, line) in lines.iter().enumerate() {
        let a = line.split(":").collect::<Vec<_>>();
        let b = a[1].split("|").collect::<Vec<_>>();
        let left = b[0];
        let right = b[1];
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
        let mut wins = 0;
        for left_number in left_numbers {
            for right_number in right_numbers.clone() {
                if left_number == right_number {
                    wins += 1;
                }
            }
        }
        for j in 0..cc[row] {
            for i in 0..wins {
                let index = row + 1 + i;
                // if index >= cc.len() {
                //     break;
                // }
                cc[index] += 1;
            }
        }
    }
    let sum = cc.iter().sum::<usize>();
    println!("{}", sum);
}
