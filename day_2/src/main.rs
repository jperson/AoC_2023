use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::u32, combinator::value,
    sequence::preceded, sequence::terminated, IResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Color {
    RED,
    BLUE,
    GREEN,
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Game {}: red: {}, blue: {}, green: {}",
            self.id, self.red, self.blue, self.green
        )
    }
}

impl Default for Game {
    fn default() -> Self {
        return Game {
            id: 0,
            red: 0,
            blue: 0,
            green: 0,
        };
    }
}

fn parse_game(game: &str) -> Game {
    let (remaining, id) = parse_game_id(game).unwrap();

    let (mut red, mut blue, mut green): (u32, u32, u32) = (0, 0, 0);

    for s in remaining.split("; ") {
        for gg in s.split(", ") {
            let (_, x) = parse_color(gg).unwrap();
            match x {
                (Color::RED, cn) if cn > red => red = cn,
                (Color::BLUE, cn) if cn > blue => blue = cn,
                (Color::GREEN, cn) if cn > green => green = cn,
                _ => (),
            }
        }
    }

    return Game {
        id,
        red,
        blue,
        green,
    };
}

fn parse_color(cube: &str) -> IResult<&str, (Color, u32)> {
    let (rm, count) = parse_cube_count(cube).unwrap();
    return alt((
        value((Color::RED, count), tag("red")),
        value((Color::BLUE, count), tag("blue")),
        value((Color::GREEN, count), tag("gree")),
    ))(rm);
}

fn parse_cube_count(cube: &str) -> IResult<&str, u32> {
    return terminated(u32, tag(" "))(cube);
}

fn parse_game_id(input: &str) -> IResult<&str, u32> {
    return terminated(preceded(tag("Game "), u32), tag(": "))(input);
}

fn main() {
    let file = File::open("./games.txt").expect("File does not exist");
    let lines: Result<Vec<_>, std::io::Error> = BufReader::new(file).lines().into_iter().collect();

    let (max_red, max_green, max_blue): (u32, u32, u32) = (12, 13, 14);
    if let Ok(ls) = lines {
        let games: Vec<Game> = ls.into_iter().map(|g| parse_game(g.as_str())).collect();
        let value: u32 = sum_game_ids(&games, max_red, max_green, max_blue);
        println!("part 1: {}", value);

        let pvalue: u32 = powers_sum(&games);
        println!("part 2: {}", pvalue);
    }
}

fn sum_game_ids(games: &Vec<Game>, max_red: u32, max_green: u32, max_blue: u32) -> u32 {
    return games.into_iter().fold(0, |acc, g| {
        acc + if g.red <= max_red && g.blue <= max_blue && g.green <= max_green {
            g.id
        } else {
            0
        }
    });
}

fn powers_sum(games: &Vec<Game>) -> u32 {
    return games
        .into_iter()
        .fold(0, |acc, g| acc + (g.red * g.blue * g.green));
}

#[cfg(test)]
mod tests {
    use crate::parse_game;

    #[test]
    fn test_parse_game() {
        let games: Vec<String> = vec![
            String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            String::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            String::from(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            ),
            String::from(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            ),
            String::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
        ];

        for g in games {
            println!("{}", parse_game(g.as_str()));
        }
    }
}
