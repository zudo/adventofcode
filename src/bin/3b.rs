use adventofcode::*;
use regex::Regex;
use std::collections::HashMap;
fn main() {
    let lines = read_lines("input.txt");
    let mut map: HashMap<(usize, usize), Vec<String>> = HashMap::new();
    let re = Regex::new(r"\d+").unwrap();
    for (row, line) in lines.iter().enumerate() {
        for m in re.find_iter(line) {
            let (start, end) = (m.start(), m.end());
            let mut vec = vec![];
            vec.push((row, start.saturating_sub(1)));
            vec.push((row, end));
            vec.extend((start.saturating_sub(1)..=end).map(|j| (row.saturating_sub(1), j)));
            vec.extend((start.saturating_sub(1)..=end).map(|j| (row + 1, j)));
            for (i, j) in vec {
                if i < lines.len() && j < lines[i].len() && lines[i].chars().nth(j).unwrap() != '.'
                {
                    map.entry((i, j)).or_default().push(m.as_str().to_string());
                }
            }
        }
    }
    let sum = map
        .values()
        .filter(|a| a.len() == 2)
        .map(|a| a[0].parse::<i64>().unwrap() * a[1].parse::<i64>().unwrap())
        .sum::<i64>();
    println!("{}", sum);
}
