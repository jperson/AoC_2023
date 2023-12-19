use num::abs;
//use std::cmp::{max, min, Ordering};
//use std::collections::*;
use std::fs::*;
use std::iter::zip;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Data {
    dir: String,
    count: i64,
    color: String,
}

fn find_corners(ds: &Vec<Data>) -> Vec<(i64, i64)> {
    let mut cur = (0, 0);
    let mut cs: Vec<(i64, i64)> = vec![cur];

    for d in ds.iter() {
        match d.dir.as_str() {
            "R" => cur = (cur.0 + d.count, cur.1),
            "L" => cur = (cur.0 - d.count, cur.1),
            "U" => cur = (cur.0, cur.1 - d.count),
            "D" => cur = (cur.0, cur.1 + d.count),
            _ => (),
        }
        cs.push(cur);
    }

    return cs;
}

fn solve(corners: &Vec<(i64, i64)>) -> i64 {
    let zcs = zip(corners.iter(), corners[1..].iter()).collect::<Vec<(&(i64, i64), &(i64, i64))>>();

    let a: i64 = zcs
        .iter()
        .fold(0, |acc, (a, b)| acc + (a.0 * b.1 - a.1 * b.0));
    let a: i64 = a / 2;
    return a;
}

fn part1(lns: &Vec<Data>) -> i64 {
    let corners = find_corners(lns);

    let a = solve(&corners);
    let dist = lns.iter().fold(0, |acc, d| acc + d.count);
    let a = abs(a) - (dist / 2) + 1;

    return a + dist;
}

fn part2(lns: &Vec<Data>) -> i64 {
    let mut vs: Vec<Data> = Vec::new();
    let mp: [&str; 4] = ["R", "D", "L", "U"];

    for l in lns {
        let idx: usize = l.color[7..8].parse::<usize>().unwrap();
        let count = i64::from_str_radix(&l.color[2..l.color.len() - 2], 16).unwrap();
        let dir = mp[idx].to_owned();
        let color = "".to_owned();
        vs.push(Data { dir, count, color });
    }

    let corners = find_corners(&vs);

    let a = solve(&corners);
    let dist = vs.iter().fold(0, |acc, d| acc + d.count);
    let a = abs(a) - (dist / 2) + 1;

    return a + dist;
}

fn main() {
    let input = read_to_string("./input.txt").expect("File does not exist");
    let lns: Vec<Data> = input
        .trim()
        .split("\n")
        .map(|s| {
            let vs = s.split(" ").collect::<Vec<&str>>();
            Data {
                dir: vs[0].to_owned(),
                count: vs[1].parse::<i64>().unwrap(),
                color: vs[2].to_owned(),
            }
        })
        .collect();

    println!("part 1: {}", part1(&lns));
    println!("part 2: {}", part2(&lns));
}
