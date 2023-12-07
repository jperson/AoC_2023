use std::collections::*;
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

fn main() {
    let file = File::open("./input.txt").expect("File does not exist");
    let lines: Result<Vec<_>, std::io::Error> = BufReader::new(file).lines().into_iter().collect();

    if let Ok(lns) = lines {
        println!("part 1: {}", part1(&lns));
        println!("part 2: {}", part2(&lns));
    }
}

fn part1(lns: &Vec<String>) -> u64 {
    let mut cs: Vec<(u64, Vec<u64>, u64)> = Vec::new();

    for l in lns.iter() {
        let sp: Vec<&str> = l.split(" ").collect();
        let (r, _) = rank(sp[0]);
        cs.push((r, hand_vec(sp[0], false), sp[1].parse::<u64>().unwrap()));
    }

    cs.sort_unstable_by_key(|i| (i.0, i.1.clone()));

    let mut acc: u64 = 0;
    for (i, (_, _, z)) in cs.iter().enumerate() {
        acc += (i as u64 + 1) * z;
    }
    return acc;
}

fn part2(lns: &Vec<String>) -> u64 {
    let mut cs: Vec<(u64, Vec<u64>, u64)> = Vec::new();

    for l in lns.iter() {
        let sp: Vec<&str> = l.split(" ").collect();
        cs.push((
            rank_j(sp[0]),
            hand_vec(sp[0], true),
            sp[1].parse::<u64>().unwrap(),
        ));
    }

    cs.sort_unstable_by_key(|i| (i.0, i.1.clone()));

    let mut acc: u64 = 0;
    for (i, (_, _, z)) in cs.iter().enumerate() {
        acc += (i as u64 + 1) * z;
    }

    return acc;
}

fn hand_vec(s: &str, joker: bool) -> Vec<u64> {
    let mut h: Vec<u64> = Vec::new();
    for c in s.chars() {
        match c {
            'A' => h.push(13),
            'K' => h.push(12),
            'Q' => h.push(11),
            'J' => h.push(if joker { 0 } else { 10 }), //h.push(10)
            'T' => h.push(9),
            '9' => h.push(8),
            '8' => h.push(7),
            '7' => h.push(6),
            '6' => h.push(5),
            '5' => h.push(4),
            '4' => h.push(3),
            '3' => h.push(2),
            '2' => h.push(1),
            _ => h.push(0),
        }
    }
    return h;
}

fn rank(s: &str) -> (u64, HashMap<char, u64>) {
    let mut m: HashMap<char, u64> = HashMap::new();

    for c in s.chars() {
        match m.get(&c) {
            Some(n) => m.insert(c, n + 1),
            None => m.insert(c, 1),
        };
    }

    let mut v: Vec<u64> = m.values().cloned().collect();
    v.sort();

    let r: u64 = match v.as_slice() {
        [1, 1, 1, 1, 1] => 1,
        [1, 1, 1, 2] => 2,
        [1, 2, 2] => 3,
        [1, 1, 3] => 4,
        [2, 3] => 5,
        [1, 4] => 6,
        [5] => 7,
        _ => 0,
    };

    return (r, m.clone());
}

fn rank_j(s: &str) -> u64 {
    let (r, m): (u64, HashMap<char, u64>) = rank(s);

    return match (r, m.get(&'J')) {
        (_, Some(4)) => 7,
        (4, Some(3)) => 6,
        (5, Some(3)) => 7,
        (2, Some(2)) => 4,
        (3, Some(2)) => 6,
        (5, Some(2)) => 7,
        (1, Some(1)) => 2,
        (2, Some(1)) => 4,
        (3, Some(1)) => 5,
        (4, Some(1)) => 6,
        (6, Some(1)) => 7,
        _ => r,
    };
}
