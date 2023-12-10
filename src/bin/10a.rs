use adventofcode::*;
fn main() {
    tracing();
    let input = read("input.txt");
    let lines = input.trim().csplit("\n");
    let vec_vec_char = lines.cmap(|line| line.chars().cv());
    for vec in vec_vec_char.iter() {
        info!(?vec);
    }
    // find starting location
    let start = 'a: {
        for (y, line) in vec_vec_char.ii().enumerate() {
            for (x, c) in line.ii().enumerate() {
                if c == 'S' {
                    break 'a (y, x);
                }
            }
        }
        unreachable!();
    };
    info!(?start);
    let mut y0 = start.0 as i32;
    let mut x0 = start.1 as i32;
    let mut y_1 = -1;
    let mut x_1 = -1;
    let mut count = 0;
    let mut char = 'S';
    loop {
        (y0, x0, char) = {
            let temp = next(vec_vec_char.clone(), y0, x0, y_1, x_1, char);
            (y_1, x_1) = (y0, x0);
            temp
        };
        info!("{} {} {} {} {}", char, y0, x0, y_1, x_1);
        count += 1;
        if char == 'S' {
            break;
        }
    }
    count /= 2;
    info!(?count);
}
fn next(
    vec_vec_char: Vec<Vec<char>>,
    y0: i32,
    x0: i32,
    y_1: i32,
    x_1: i32,
    char: char,
) -> (i32, i32, char) {
    for (dy, dx) in a(char) {
        let y1 = y0 + dy;
        let x1 = x0 + dx;
        if (y1, x1) == (y_1, x_1) {
            continue;
        }
        let char_next = vec_vec_char[y1 as usize][x1 as usize];
        if a(char_next).contains(&(-dy, -dx)) {
            return (y1, x1, char_next);
        }
    }
    unreachable!()
}
fn a(char: char) -> Vec<(i32, i32)> {
    match char {
        '|' => vec![(-1, 0), (1, 0)],
        '-' => vec![(0, -1), (0, 1)],
        'L' => vec![(0, 1), (-1, 0)],
        'J' => vec![(-1, 0), (0, -1)],
        '7' => vec![(0, -1), (1, 0)],
        'F' => vec![(1, 0), (0, 1)],
        'S' => vec![(1, 0), (0, 1), (-1, 0), (0, -1)],
        _ => vec![],
    }
}
