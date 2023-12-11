use std::cmp::{max, min};
use std::io::prelude::*;
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
    let (rows, cols) = expand(lns);
    let m = parse_map(lns);
    return pairs_dist(1, m, rows, cols);
}

fn part2(lns: &Vec<String>) -> i64 {
    let (rows, cols) = expand(lns);
    let m = parse_map(lns);
    return pairs_dist(1000000, m, rows, cols);
}

fn pairs_dist(f: i64, m: Vec<(i64, i64)>, rows: Vec<i64>, cols: Vec<i64>) -> i64 {
    let m2 = m.clone();
    let m2l = m2.len();
    let mut total: i64 = 0;

    for (i, p1) in m.iter().enumerate() {
        for p2 in m2[i + 1..m2l].iter() {
            let (x1, x2) = (min(p1.0, p2.0), max(p1.0, p2.0));
            let (y1, y2) = (min(p1.1, p2.1), max(p1.1, p2.1));

            let yacc: i64 = rows
                .iter()
                .filter(|r| r > &&x1 && r < &&x2)
                .collect::<Vec<&i64>>()
                .len() as i64;
            let ydist: i64 = (y2 - y1 - yacc) + (yacc * f);

            let xacc: i64 = cols
                .iter()
                .filter(|c| c > &&y1 && c < &&y2)
                .collect::<Vec<&i64>>()
                .len() as i64;
            let xdist: i64 = (x2 - x1 - xacc) + (xacc * f);

            total += xdist + ydist;
        }
    }

    return total;
}

fn expand(lns: &Vec<String>) -> (Vec<i64>, Vec<i64>) {
    let mut rows: Vec<i64> = Vec::new();
    let mut cols: Vec<i64> = Vec::new();

    for (i, l) in lns.iter().enumerate() {
        if !l.contains("#") {
            rows.push(i as i64);
        }
    }

    for i in 0..lns[0].len() {
        let mut emp: bool = true;
        for l in lns.iter() {
            if l.chars().nth(i).unwrap() == '#' {
                emp = false;
                break;
            }
        }
        if emp {
            cols.push(i as i64);
        }
    }
    return (rows, cols);
}

fn parse_map(lns: &Vec<String>) -> Vec<(i64, i64)> {
    let mut m: Vec<(i64, i64)> = Vec::new();

    for (i, l) in lns.iter().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == '#' {
                m.push((i as i64, j as i64));
            }
        }
    }

    return m;
}
