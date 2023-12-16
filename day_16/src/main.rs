use std::collections::*;
use std::fs::*;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

fn part1(lns: &Vec<Vec<char>>) -> i64 {
    return traverse(&lns, 0, 0, Dir::Right);
}

fn part2(lns: &Vec<Vec<char>>) -> i64 {
    let mut max: i64 = 0;
    for x in 0..lns[0].len() {
        let z = traverse(&lns, x as i64, 0, Dir::Down);
        if z > max {
            max = z;
        }

        let z = traverse(&lns, x as i64, (lns.len() as i64) - 1, Dir::Up);
        if z > max {
            max = z;
        }
    }

    for y in 0..lns.len() {
        let z = traverse(&lns, 0, y as i64, Dir::Right);
        if z > max {
            max = z;
        }

        let z = traverse(&lns, (lns[0].len() as i64) - 1, y as i64, Dir::Left);
        if z > max {
            max = z;
        }
    }
    return max;
}

fn traverse(lns: &Vec<Vec<char>>, x: i64, y: i64, d: Dir) -> i64 {
    let mut q: VecDeque<(i64, i64, Dir)> = VecDeque::new();
    let mut m: HashMap<(i64, i64, Dir), i64> = HashMap::new();
    let mut c: HashMap<(i64, i64), i64> = HashMap::new();
    let mut total: i64 = 0;

    let ymax: i64 = lns.len() as i64;
    let ymax = ymax - 1;
    let xmax: i64 = lns[0].len() as i64;
    let xmax = xmax - 1;

    q.push_back((x, y, d.clone()));

    while !q.is_empty() {
        let (x, y, d) = q.pop_front().unwrap();

        if m.contains_key(&(x, y, d.clone())) {
            continue;
        }

        if x > xmax || x < 0 || y > ymax || y < 0 {
            continue;
        }

        if !c.contains_key(&(x, y)) {
            total += 1;
        }

        m.insert((x, y, d.clone()), 1);
        c.insert((x, y), 1);

        match (lns[y as usize][x as usize], &d) {
            ('|', Dir::Down) => {
                q.push_back((x, y + 1, Dir::Down));
            }
            ('|', Dir::Up) => {
                q.push_back((x, y - 1, Dir::Up));
            }
            ('|', Dir::Left) | ('|', Dir::Right) => {
                q.push_back((x, y + 1, Dir::Down));
                q.push_back((x, y - 1, Dir::Up));
            }
            ('-', Dir::Left) => {
                q.push_back((x - 1, y, Dir::Left));
            }
            ('-', Dir::Right) => {
                q.push_back((x + 1, y, Dir::Right));
            }
            ('-', Dir::Up) | ('-', Dir::Down) => {
                q.push_back((x - 1, y, Dir::Left));
                q.push_back((x + 1, y, Dir::Right));
            }
            ('/', Dir::Left) | ('\\', Dir::Right) => {
                q.push_back((x, y + 1, Dir::Down));
            }
            ('/', Dir::Right) | ('\\', Dir::Left) => {
                q.push_back((x, y - 1, Dir::Up));
            }
            ('/', Dir::Up) | ('\\', Dir::Down) => {
                q.push_back((x + 1, y, Dir::Right));
            }
            ('/', Dir::Down) | ('\\', Dir::Up) => {
                q.push_back((x - 1, y, Dir::Left));
            }
            ('.', Dir::Down) => {
                q.push_back((x, y + 1, Dir::Down));
            }
            ('.', Dir::Up) => {
                q.push_back((x, y - 1, Dir::Up));
            }
            ('.', Dir::Left) => {
                q.push_back((x - 1, y, Dir::Left));
            }
            ('.', Dir::Right) => {
                q.push_back((x + 1, y, Dir::Right));
            }
            _ => (),
        }
    }

    return total;
}

fn main() {
    let input = read_to_string("./input.txt").expect("File does not exist");
    let lns: Vec<Vec<char>> = input
        .trim()
        .split("\n")
        .map(|cs| cs.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    println!("part 1: {}", part1(&lns));
    println!("part 2: {}", part2(&lns));
}
