use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use nom::{
    bytes::complete::tag, character::complete::space0, character::complete::space1,
    character::complete::u32, multi::many0, sequence::preceded, sequence::terminated, IResult,
};

fn main() {
    let file = File::open("./input.txt").expect("File does not exist");
    let lines: Result<Vec<_>, std::io::Error> = BufReader::new(file).lines().into_iter().collect();

    if let Ok(lns) = lines {
        let mut count: u32 = 0;
        for l in lns.iter() {
            let wins = parse_card(&l);
            if wins > 0 {
                count += u32::pow(2, wins - 1);
            }
        }
        println!("part 1: {}", count);

        let mut ccounts: Vec<u32> = vec![1; lns.len()];
        for (i, l) in lns.iter().enumerate() {
            let mut wins = parse_card(&l);

            let mut start = i + 1;

            while wins > 0 {
                ccounts[start] += ccounts[i];
                wins -= 1;
                start += 1;
            }
        }
        println!("part 2: {}", ccounts.iter().sum::<u32>());
    }
}

fn parse_game(s: &str) -> IResult<&str, u32> {
    return terminated(preceded(tag("Card"), preceded(space1, u32)), tag(":"))(s);
}

fn parse_winners(s: &str) -> IResult<&str, Vec<u32>> {
    return terminated(many0(terminated(preceded(space0, u32), space1)), tag("|"))(s);
}

fn parse_nums(s: &str) -> IResult<&str, Vec<u32>> {
    return many0(preceded(space0, u32))(s);
}

fn parse_card(s: &str) -> u32 {
    let (rest, _) = parse_game(s).unwrap();
    let (rest, winners) = parse_winners(rest).unwrap();
    let (_, nums) = parse_nums(rest).unwrap();

    let mut count: u32 = 0;
    for w in winners {
        if nums.contains(&w) {
            count += 1;
        }
    }

    return count;
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_fn() {
        let input = vec![
            String::from("Card 100: 22 74 77 49  4 95 63 44 52 56 |  7 86  3 46 26 23 78 97 43 16 91 30 92 77 18 51 41 15  1 21 19 61 88 14 27"),
            String::from("Card   1: 82 15 37 75 85 51 99 18 17  2 |  8 17 54 14 18 99  4 85 51 49 91 15 82 24 75 25 69 61 52 58 37 87  2 22 28"),
            String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            String::from("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
            String::from("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
            String::from("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            String::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            String::from("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
        ];

        let (rest, game) = parse_game(&input[4]).unwrap();
        println!("game: {:?}, rest: {:?}", game, rest);

        let (rest, winners) = parse_winners(&rest).unwrap();
        println!("winners: {:?}", winners);

        let (_, nums) = parse_nums(&rest).unwrap();
        println!("nums: {:?}", nums);
    }
}
