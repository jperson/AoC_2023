use num::abs;
use std::collections::VecDeque;
use std::io::prelude::*;
use std::iter::zip;
use std::{fs::File, io::BufReader};

fn main() {
    let file = File::open("./input.txt").expect("File does not exist");
    let lines: Result<Vec<String>, std::io::Error> =
        BufReader::new(file).lines().into_iter().collect();

    if let Ok(lns) = lines {
        println!("part 1: {}", part1(&lns));
        println!("part 2: {}", part2(&lns));
    }
}

fn part1(lns: &Vec<String>) -> i64 {
    let (m, start) = parse_map(lns);

    return bfs(m, start);
}

fn part2(lns: &Vec<String>) -> i64 {
    let (m, start) = parse_map(lns);
    let (dist, cs) = find_corners(m, start);

    let csl = cs.len();
    let v: Vec<_> = zip(&cs[0..csl], &cs[1..csl]).collect();

    //shoelace formula
    let a: i64 = v
        .iter()
        .fold(0, |acc, (a, b)| acc + (a.0 * b.1 - a.1 * b.0));
    let a: i64 = a / 2;

    /*
     * Pick's theorem
     * A = i + (b/2) - 1
     * i = A - (b/2) + 1
     */
    return abs(a) - (dist / 2) + 1;
}

fn bfs(mut m: Vec<Vec<char>>, start: (usize, usize)) -> i64 {
    let mut q: VecDeque<Vec<(usize, usize)>> = VecDeque::new();
    let mut dist: i64 = -1;

    q.push_back(vec![start]);
    while let Some(rv) = q.pop_front() {
        let mut row: Vec<(usize, usize)> = Vec::new();

        for n in rv.iter() {
            if ".*".contains(m[n.0][n.1]) {
                continue;
            }

            //above
            if "|F7".contains(m[n.0 - 1][n.1]) {
                row.push((n.0 - 1, n.1));
            }

            //below
            if "|LJ".contains(m[n.0 + 1][n.1]) {
                row.push((n.0 + 1, n.1));
            }

            //left
            if "-LF".contains(m[n.0][n.1 - 1]) {
                row.push((n.0, n.1 - 1));
            }

            //right
            if "-J7".contains(m[n.0][n.1 + 1]) {
                row.push((n.0, n.1 + 1));
            }

            m[n.0][n.1] = '*';
        }

        if !row.is_empty() {
            q.push_back(row);
        }
        dist += 1;
    }

    return dist;
}

fn parse_map(lns: &Vec<String>) -> (Vec<Vec<char>>, (usize, usize)) {
    let height = lns.len() + 2;
    let width = lns[0].len() + 2;

    let mut m: Vec<Vec<char>> = vec![vec!['.'; width]; height];
    let mut start: (usize, usize) = (0, 0);

    for (i, s) in lns.iter().enumerate() {
        for (j, c) in s.chars().enumerate() {
            m[i + 1][j + 1] = c;
            if c == 'S' {
                start = (i + 1, j + 1);
            }
        }
    }

    return (m, start);
}

fn find_corners(mut m: Vec<Vec<char>>, start: (usize, usize)) -> (i64, Vec<(i64, i64)>) {
    let mut cs: Vec<(i64, i64)> = Vec::new();
    cs.push((start.0 as i64, start.1 as i64));

    let mut dist: i64 = 0;
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_front(start);

    while let Some((x, y)) = q.pop_front() {
        if ".*".contains(m[x][y]) {
            continue;
        }
        dist += 1;
        cs.push((x as i64, y as i64));

        match m[x][y] {
            'F' => {
                if "|LJ".contains(m[x + 1][y]) {
                    q.push_front((x + 1, y));
                }

                if "-7J".contains(m[x][y + 1]) {
                    q.push_front((x, y + 1));
                }
            }
            'L' => {
                if "|F7".contains(m[x - 1][y]) {
                    q.push_front((x - 1, y));
                }

                if "-J7".contains(m[x][y + 1]) {
                    q.push_front((x, y + 1));
                }
            }
            '7' => {
                if "|LJ".contains(m[x + 1][y]) {
                    q.push_front((x + 1, y));
                }

                if "-FL".contains(m[x][y - 1]) {
                    q.push_front((x, y - 1));
                }
            }
            'J' => {
                if "|7F".contains(m[x - 1][y]) {
                    q.push_front((x - 1, y));
                }

                if "-LF".contains(m[x][y - 1]) {
                    q.push_front((x, y - 1));
                }
            }
            '|' => {
                if "|LJ".contains(m[x + 1][y]) {
                    q.push_front((x + 1, y));
                }

                if "|F7".contains(m[x - 1][y]) {
                    q.push_front((x - 1, y));
                }
            }
            '-' => {
                if "-J7".contains(m[x][y + 1]) {
                    q.push_front((x, y + 1));
                }
                if "-FL".contains(m[x][y - 1]) {
                    q.push_front((x, y - 1));
                }
            }
            'S' => {
                if "-7J".contains(m[x][y + 1]) {
                    q.push_front((x, y + 1));
                }
                if "-FL".contains(m[x][y - 1]) {
                    q.push_front((x, y - 1));
                }
                if "|LJ".contains(m[x + 1][y]) {
                    q.push_front((x + 1, y));
                }
                if "|F7".contains(m[x - 1][y]) {
                    q.push_front((x - 1, y));
                }
            }
            _ => (),
        }

        m[x][y] = '*';
    }

    cs.push((start.0 as i64, start.1 as i64));

    return (dist, cs);
}
