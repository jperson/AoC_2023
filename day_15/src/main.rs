use std::fs::*;

fn part1(lns: &Vec<Vec<char>>) -> i64 {
    let mut total: i64 = 0;
    for l in lns.iter() {
        total += hash(&l);
    }
    return total;
}

fn part2(lns: &Vec<Vec<char>>) -> i64 {
    let mut bs: Vec<Vec<(String, i32)>> = vec![Vec::new(); 256];

    for l in lns.iter() {
        let s = l.iter().collect::<String>();

        if l.contains(&'=') {
            let ss: Vec<String> = s.split("=").map(|x| String::from(x)).collect();

            let cmd = ss[0].chars().collect::<Vec<char>>();
            let focal = ss[1].parse::<i32>().unwrap();
            let idx = hash(&cmd) as usize;

            'found: {
                for (i, b) in bs[idx as usize].clone().iter().enumerate() {
                    if b.0 == ss[0] {
                        bs[idx][i].1 = focal;
                        break 'found;
                    }
                }
                bs[idx].push((ss[0].clone(), focal));
            }
        } else if s.ends_with("-") {
            let ss: Vec<String> = s.split("-").map(|x| String::from(x)).collect();
            let cmd = ss[0].chars().collect::<Vec<char>>();
            let idx = hash(&cmd) as usize;

            for (i, r) in bs[idx].clone().iter().enumerate() {
                if r.0 == ss[0] {
                    bs[idx].remove(i);
                    break;
                }
            }
        }
    }

    let mut total: i64 = 0;
    for (i, b) in bs.iter().enumerate() {
        for (x, e) in b.iter().enumerate() {
            total += (i as i64 + 1) * (x as i64 + 1) * e.1 as i64;
        }
    }
    return total;
}

fn hash(v: &Vec<char>) -> i64 {
    let mut current: i64 = 0;
    for c in v.iter() {
        current += *c as i64;
        current *= 17;
        current %= 256;
    }
    return current;
}

fn main() {
    let input = read_to_string("./input.txt").expect("File does not exist");
    let lns: Vec<Vec<char>> = input
        .trim()
        .split(",")
        .map(|cs| cs.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    println!("part 1: {}", part1(&lns));
    println!("part 2: {}", part2(&lns));
}
