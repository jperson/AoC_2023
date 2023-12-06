use std::io::prelude::*;
use std::iter::zip;
use std::{fs::File, io::BufReader};

use nom::{
    bytes::complete::tag, character::complete::space0, character::complete::u64, multi::many0,
    sequence::preceded, IResult,
};

fn main() {
    let file = File::open("./input.txt").expect("File does not exist");
    let lines: Result<Vec<_>, std::io::Error> = BufReader::new(file).lines().into_iter().collect();

    if let Ok(lns) = lines {
        let (_, ts) = parse_times(&lns[0]).unwrap();
        let (_, ds) = parse_dist(&lns[1]).unwrap();
        let rs: Vec<(u64, u64)> = zip(ts, ds).collect();

        println!("{}", races(rs));
    }
}

fn parse_times(s: &str) -> IResult<&str, Vec<u64>> {
    return preceded(tag("Time:"), many0(preceded(space0, u64)))(s);
}

fn parse_dist(s: &str) -> IResult<&str, Vec<u64>> {
    return preceded(tag("Distance:"), many0(preceded(space0, u64)))(s);
}

fn races(rs: Vec<(u64, u64)>) -> u64 {
    let mut total: u64 = 1;

    for (t, d) in rs.into_iter() {
        total *= (1..t)
            .into_iter()
            .fold(0, |acc, h| acc + if h * (t - h) > d { 1 } else { 0 });
    }
    return total;
}
