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
    let mut acc: Vec<i64> = Vec::new();

    for l in lns.iter() {
        let mut lv: i64 = 0;
        let mut nums: Vec<i64> = l
            .split(" ")
            .into_iter()
            .map(|v| v.parse::<i64>().unwrap())
            .collect();

        lv += nums.last().unwrap();

        loop {
            let c = nums.len();
            let n: Vec<(&i64, &i64)> = zip(&nums[0..c], &nums[1..c]).collect();
            let x: Vec<i64> = n.iter().map(|a| a.1 - a.0).collect();

            if x.iter().all(|&v| v == 0) {
                acc.push(lv);
                break;
            } else {
                lv += x.last().unwrap();
                nums = x;
            }
        }
    }

    return acc.iter().sum();
}

fn part2(lns: &Vec<String>) -> i64 {
    let mut acc: Vec<i64> = Vec::new();

    for l in lns.iter() {
        let mut lv: Vec<i64> = Vec::new();
        let mut nums: Vec<i64> = l
            .split(" ")
            .into_iter()
            .map(|v| v.parse::<i64>().unwrap())
            .collect();

        lv.push(*nums.first().unwrap());

        loop {
            let c = nums.len();
            let n: Vec<(&i64, &i64)> = zip(&nums[0..c], &nums[1..c]).collect();
            let x: Vec<i64> = n.iter().map(|a| a.1 - a.0).collect();

            if x.iter().all(|&v| v == 0) {
                lv.reverse();
                let nv: i64 = lv[1..lv.len()].iter().fold(lv[0], |acc, l| l - acc);
                acc.push(nv);
                break;
            } else {
                lv.push(*x.first().unwrap());
                nums = x;
            }
        }
    }

    return acc.iter().sum();
}
