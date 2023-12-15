use std::collections::*;
use std::fs::*;

fn part1(lns: &Vec<Vec<char>>) -> i64 {
    let mut tx = rot(&lns);
    roll(&mut tx);

    for _ in 0..3 {
        tx = rot(&tx);
    }

    return count(&tx);
}

fn part2(lns: &mut Vec<Vec<char>>) -> i64 {
    let mut cache: HashMap<String, usize> = HashMap::new();
    cache.insert(to_key(lns), 0);

    for c in 0..1000000000 {
        *lns = cycle(lns);

        if cache.contains_key(&to_key(lns)) {
            let t = cache.get(&to_key(lns)).unwrap();
            let cx = (1000000000 - c) % (c - t) - 1;
            for _ in 0..cx {
                *lns = cycle(lns);
            }
            break;
        }
        cache.insert(to_key(lns), c);
    }

    return count(&lns);
}

fn rot(vs: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut ts: Vec<Vec<char>> = Vec::new();

    for i in 0..vs[0].len() {
        let mut r: Vec<char> = Vec::new();
        for v in vs.iter() {
            r.push(v[i]);
        }
        r.reverse();
        ts.push(r);
    }
    return ts;
}

fn count(lns: &Vec<Vec<char>>) -> i64 {
    let mut fac = lns.len();
    let mut total = 0;
    for r in lns.iter() {
        let x = r.iter().filter(|&c| *c == 'O').count();
        total += fac * x;
        fac -= 1;
    }
    return total as i64;
}

fn roll(vs: &mut Vec<Vec<char>>) {
    let ln = vs[0].len() - 1;

    for l in 0..vs.len() {
        let mut swap = ln;
        while vs[l][swap] != '.' {
            swap -= 1;
        }

        for r in (0..swap).rev() {
            if vs[l][r] == 'O' {
                vs[l].swap(swap, r);

                if swap > 0 {
                    swap -= 1;
                }
            } else if vs[l][r] == '#' && r > 0 {
                swap = r - 1;
            }
        }
    }
}

fn to_key(vs: &mut Vec<Vec<char>>) -> String {
    return vs.iter().flatten().collect::<String>();
}

fn cycle(vs: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut tx = rot(&vs);
    roll(&mut tx);

    tx = rot(&tx);
    roll(&mut tx);

    tx = rot(&tx);
    roll(&mut tx);

    tx = rot(&tx);
    roll(&mut tx);

    return tx;
}

fn main() {
    let input = read_to_string("./input.txt").expect("File does not exist");
    let lns: Vec<Vec<char>> = input
        .trim()
        .split("\n")
        .map(|cs| cs.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    println!("part 1: {}", part1(&lns));
    println!("part 2: {}", part2(&mut lns.clone()));
}
