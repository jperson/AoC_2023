use memoize::memoize;
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
    let springs: Vec<(Vec<char>, Vec<usize>)> = parse_rows(lns);

    let mut total: i64 = 0;

    for (s, n) in springs.iter() {
        let t = solve(s.to_owned(), n.to_owned());
        total += t;
    }
    return total;
}

fn part2(lns: &Vec<String>) -> i64 {
    let springs: Vec<(Vec<char>, Vec<usize>)> = parse_rows(lns);

    let mut total: i64 = 0;

    for (s, n) in springs.iter() {
        let qss = s.iter().collect::<String>();
        let q: Vec<char> = vec![qss.as_str()].repeat(5).join("?").chars().collect();

        let t = solve(q, n.repeat(5));
        total += t;
    }
    return total;
}

#[memoize]
fn solve(s: Vec<char>, ns: Vec<usize>) -> i64 {
    let mut result = 0;

    if s.is_empty() {
        return if ns.is_empty() { 1 } else { 0 };
    }

    if ns.is_empty() {
        return if s.contains(&'#') { 0 } else { 1 };
    }

    if ".?".contains(s[0]) {
        result += solve(s[1..].to_vec(), ns.to_vec());
    }

    if "#?".contains(s[0])
        && ns[0] <= s.len()
        && !&s[..ns[0]].contains(&'.')
        && (ns[0] == s.len() || s[ns[0]] != '#')
    {
        if ns[0] + 1 > s.len() {
            result += if ns[1..].is_empty() { 1 } else { 0 };
        } else {
            result += solve(s[ns[0] + 1..].to_vec(), ns[1..].to_vec());
        }
    }

    return result;
}

fn parse_rows(lns: &Vec<String>) -> Vec<(Vec<char>, Vec<usize>)> {
    let mut r: Vec<(Vec<char>, Vec<usize>)> = Vec::new();
    for l in lns.iter() {
        let s: Vec<&str> = l.split(' ').collect();
        let ns: Vec<&str> = s[1].split(',').collect();
        let rs: Vec<usize> = ns
            .iter()
            .map(|x| x.to_string().parse::<usize>().unwrap())
            .collect();

        r.push((s[0].chars().collect(), rs));
    }

    return r;
}
