use adventofcode::*;
fn main() {
    tracing();
    let input = read("input.txt");
    let lines = input.trim().csplit("\n");
    let vec_vec_char = lines.cmap(|line| line.chars().cv());
    // for vec in vec_vec_char.iter() {
    //     info!(?vec);
    // }
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
    // info!(?start);
    let mut y0 = start.0 as i32;
    let mut x0 = start.1 as i32;
    let mut y_1 = -1;
    let mut x_1 = -1;
    let mut count = 0;
    let mut char = 'S';
    let mut dy;
    let mut dx;
    let mut inner = 0;
    let mut remember = HashSet::new();
    loop {
        (y0, x0, char, dy, dx) = {
            let temp = next(vec_vec_char.clone(), y0, x0, y_1, x_1, char);
            (y_1, x_1) = (y0, x0);
            temp
        };
        remember.insert((y0, x0));
        // info!("{} {} {} {} {}", char, y0, x0, y_1, x_1);
        // if dx == 0 {
        //     if let Some(vec) = vec_vec_char.get((y0 - 1) as usize) {
        //         if vec[x0 as usize] == '.' {
        //             info!(?y0, ?x0);
        //             inner += 1;
        //         }
        //     }
        // }
        // if dy == 0 {
        //     if let Some(char) = vec_vec_char[y0 as usize].get((x0 - 1) as usize) {
        //         if *char == '.' {
        //             inner += 1;
        //         }
        //     }
        // }
        count += 1;
        if char == 'S' {
            break;
        }
    }
    count /= 2;
    // info!(?count);
    // let mut bigger = vec![vec![]; vec_vec_char.len() * 3];
    // for i in 0..vec_vec_char[0].len() * 3 {
    //     bigger[i].push(0);
    // }
    let mut bigger = vec![];
    for y in 0..vec_vec_char.len() {
        bigger.push(vec![]);
        for x in 0..vec_vec_char[0].len() {
            // for z in 0..3 {
            //     bigger[y][x].push(vec![]);
            //     for _ in 0..3 {
            //         bigger[y][x][z].push(0);
            //     }
            // }
            if remember.contains(&(y as i32, x as i32)) {
                let char = vec_vec_char[y][x];
                // info!(?char);
                let arr = match char {
                    '|' => [[0, 1, 0], [0, 1, 0], [0, 1, 0]],
                    '-' => [[0, 0, 0], [1, 1, 1], [0, 0, 0]],
                    'L' => [[0, 1, 0], [0, 1, 1], [0, 0, 0]],
                    'J' => [[0, 1, 0], [1, 1, 0], [0, 0, 0]],
                    '7' => [[0, 0, 0], [1, 1, 0], [0, 1, 0]],
                    'F' => [[0, 0, 0], [0, 1, 1], [0, 1, 0]],
                    'S' => [[0, 1, 0], [1, 1, 1], [0, 1, 0]],
                    _ => unreachable!(),
                };
                bigger[y].push(arr);
            } else {
                bigger[y].push([[0, 0, 0], [0, 0, 0], [0, 0, 0]]);
            }
            // if remember.contains(&(y as i32, x as i32)) {
            //     print!("0");
            // } else {
            //     print!(" ");
            // }
        }
        println!();
    }
    // info!(?bigger);
    let mut asdf = vec![];
    for i in 0..bigger.len() {
        let mut vec_vec = vec![vec![]; 3];
        for j in 0..bigger[i].len() {
            for k in 0..bigger[i][j].len() {
                for l in 0..bigger[i][j][k].len() {
                    if bigger[i][j][k][l] == 1 {
                        vec_vec[k].push('#');
                    } else {
                        vec_vec[k].push(' ');
                    }
                }
            }
        }
        for vec in vec_vec.iter() {
            asdf.push(vec.to_vec());
            // println!("{}", vec.ii().join(""));
        }
    }
    // for vec in asdf.iter() {
    //     info!(?vec);
    // }
    let mut neighbors: Vec<(i32, i32)> = vec![(0, 0)];
    loop {
        let temp = neighbors.clone();
        neighbors.clear();
        for neighbor in temp {
            for (dy, dx) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let y = (neighbor.0 + dy) as usize;
                let x = (neighbor.1 + dx) as usize;
                // info!(?y, ?x, "{}", neighbors.len());
                if let Some(char) = asdf.get(y).and_then(|vec| vec.get(x)) {
                    // info!(?char);
                    if char != &'#' {
                        neighbors.push((y as i32, x as i32));
                    }
                    asdf[y][x] = '#';
                }
            }
        }
        if neighbors.is_empty() {
            break;
        }
    }
    for vec in asdf.iter() {
        // info!(?vec);
        println!("{}", vec.ii().join(""));
    }
    let mut count = vec![vec![0; vec_vec_char[0].len()]; vec_vec_char.len()];
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..asdf.len() / 3 {
                for l in 0..asdf[k].len() / 3 {
                    let char = asdf[k * 3 + i][l * 3 + j];
                    if char == '#' {
                        count[k][l] += 1;
                    }
                }
            }
        }
    }
    for vec in count.iter() {
        println!(
            "{}",
            vec.ii()
                .map(|x| if x == 0 {
                    " ".to_string()
                } else {
                    x.to_string()
                })
                .join("")
        );
    }
    let mut sum = 0;
    for cc in count {
        for c in cc {
            if c == 0 {
                sum += 1;
            }
        }
    }
    info!(?sum);
}
fn next(
    vec_vec_char: Vec<Vec<char>>,
    y0: i32,
    x0: i32,
    y_1: i32,
    x_1: i32,
    char: char,
) -> (i32, i32, char, i32, i32) {
    for (dy, dx) in a(char) {
        let y1 = y0 + dy;
        let x1 = x0 + dx;
        if (y1, x1) == (y_1, x_1) {
            continue;
        }
        let char_next = vec_vec_char[y1 as usize][x1 as usize];
        if a(char_next).contains(&(-dy, -dx)) {
            return (y1, x1, char_next, dy, dx);
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
