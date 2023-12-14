use std::cmp::min;
use std::fs::*;

fn main() {
    let input = read_to_string("./input.txt").expect("File does not exist");
    let lns: Vec<&str> = input.split("\n\n").collect();

    println!("part 1: {}", part1(&lns, 0));
    println!("part 2: {}", part1(&lns, 1));
}

fn part1(lns: &Vec<&str>, c: i32) -> i32 {
    let mut total: i32 = 0;
    for l in lns.iter() {
        let m = l
            .trim()
            .split("\n")
            .map(|n| n.chars().map(|c| if c == '#' { 1 } else { 0 }).collect())
            .collect::<Vec<Vec<i32>>>();

        total += 100 * count(&m, c);
        total += count(&transpose(&m), c);
    }
    return total;
}

fn transpose(vs: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut ts: Vec<Vec<i32>> = Vec::new();

    for i in 0..vs[0].len() {
        let mut r: Vec<i32> = Vec::new();
        for v in vs.iter() {
            r.push(v[i]);
        }
        ts.push(r);
    }
    return ts;
}

fn count(m: &Vec<Vec<i32>>, c: i32) -> i32 {
    for r in 1..m.len() {
        let top = min(r, m.len() - r);

        let s = (0..top).fold(0, |acc, i| {
            acc + m[r + i]
                .iter()
                .zip(&m[r - i - 1])
                .filter(|&(a, b)| a != b)
                .count()
        });
        if s as i32 == c {
            return r as i32;
        }
    }
    return 0;
}
