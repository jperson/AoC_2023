use num::integer::lcm;
use std::collections::*;
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

fn part1(lns: &Vec<String>) -> u64 {
    let moves: Vec<char> = lns[0].chars().collect();
    let m = parse_map(lns);

    let mut i = 0;
    let mut state: &str = "AAA";
    let mut acc: u64 = 0;

    while state != "ZZZ" {
        match moves[i] {
            'L' => state = &m[state].0,
            'R' => state = &m[state].1,
            _ => (),
        }
        i = (i + 1) % moves.len();
        acc += 1;
    }
    return acc;
}

fn part2(lns: &Vec<String>) -> u64 {
    let moves: Vec<char> = lns[0].chars().collect();
    let m = parse_map(lns);
    let states: Vec<&&str> = m.keys().filter(|&&k| k.ends_with('A')).collect();

    let mut i = 0;
    let mut vacc: Vec<u64> = Vec::new();

    for state in states.iter() {
        let mut cur_state: &str = &state;
        let mut acc: u64 = 0;

        while !cur_state.ends_with('Z') {
            match moves[i] {
                'L' => cur_state = &m[cur_state].0,
                'R' => cur_state = &m[cur_state].1,
                _ => (),
            }
            i = (i + 1) % moves.len();
            acc += 1;
        }
        vacc.push(acc);
    }

    return vacc[1..vacc.len()]
        .iter()
        .fold(vacc[0], |acc, n| lcm(acc, *n));
}

fn parse_map(s: &Vec<String>) -> HashMap<&str, (&str, &str)> {
    let mut m: HashMap<&str, (&str, &str)> = HashMap::new();

    for l in &s[2..] {
        let values: Vec<&str> = l.split(" = ").collect();
        let rls: Vec<&str> = values[1].split(", ").collect();

        let a: &str = rls[0];
        let b: &str = rls[1];

        m.insert(&values[0], (&a[1..=3], &b[0..=2]));
    }
    return m;
}
