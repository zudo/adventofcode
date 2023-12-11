use adventofcode::*;
fn main() {
    tracing();
    let input = read("input.txt");
    let lines = input.trim().csplit("\n");
    let mut vec_vec_char = lines.cmap(|line| line.chars().cv());
    for vec_char in vec_vec_char.ii() {
        info!(?vec_char);
    }
    let h = vec_vec_char.len();
    let w = vec_vec_char[0].len();
    let mut expand_y = vec![];
    'a: for y in 0..h {
        for x in 0..w {
            let c = vec_vec_char[y][x];
            if c == '#' {
                continue 'a;
            }
        }
        expand_y.push(y);
    }
    let mut expand_x = vec![];
    'a: for x in 0..w {
        for y in 0..h {
            let c = vec_vec_char[y][x];
            if c == '#' {
                continue 'a;
            }
        }
        expand_x.push(x);
    }
    let h = vec_vec_char.len();
    let w = vec_vec_char[0].len();
    let galaxies = input.chars().filter(|c| *c == '#').count();
    info!(?galaxies);
    let mut i = 0;
    let vec_vec_int = vec_vec_char.cmap(|vec_char| {
        vec_char
            .cmap(|c| {
                if c == '#' {
                    i += 1;
                    i
                } else {
                    0
                }
            })
            .cv()
    });
    for vec_int in vec_vec_int.ii() {
        println!(
            "{}",
            vec_int
                .ii()
                .map(|n| if n == 0 {
                    '.'
                } else {
                    n.to_string().chars().next().unwrap()
                })
                .join("")
        );
    }
    let mut pairs = Vec::new();
    for i in 1..galaxies + 1 {
        for j in i + 1..galaxies + 1 {
            pairs.push((i, j));
        }
    }
    info!(?pairs, len = pairs.len());
    let mut sum = 0;
    for (a, b) in pairs {
        let cords = |n: usize| -> (usize, usize) {
            'a: {
                for y in 0..h {
                    for x in 0..w {
                        if vec_vec_int[y][x] == n {
                            break 'a (y as usize, x as usize);
                        }
                    }
                }
                unreachable!()
            }
        };
        let (ay, ax) = cords(a);
        let (by, bx) = cords(b);
        let dy = (ay as isize - by as isize).abs();
        let dx = (ax as isize - bx as isize).abs();
        let mut distance = dy + dx;
        let space = 1000_000 - 1;
        for y in expand_y.ii() {
            if ay < y && by > y {
                distance += space;
            }
            if ay > y && by < y {
                distance += space;
            }
        }
        for x in expand_x.ii() {
            if ax < x && bx > x {
                distance += space;
            }
            if ax > x && bx < x {
                distance += space;
            }
        }
        // if dy == 0 {
        //     distance -= 1;
        // }
        // if dx == 0 {
        //     distance -= 1;
        // }
        info!(?a, ?b, ?distance);
        sum += distance;
    }
    info!(h, w);
    info!(?sum);
}
