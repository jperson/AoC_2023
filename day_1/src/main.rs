use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn replace_words(s: String) -> String {
    let mut outs = String::from(&s);
    outs = outs.replace("one", "o1e");
    outs = outs.replace("two", "t2o");
    outs = outs.replace("three", "t3e");
    outs = outs.replace("four", "f4");
    outs = outs.replace("five", "f5e");
    outs = outs.replace("six", "s6");
    outs = outs.replace("seven", "s7n");
    outs = outs.replace("eight", "8t");
    outs = outs.replace("nine", "n9e");

    if outs == String::from(&s) {
        return outs;
    } else {
        return replace_words(outs);
    }
}

fn clean_str(s: String) -> String {
    let mut outs = String::from(s);
    outs.retain(|c| r#"0123456789"#.contains(c));
    return outs;
}

fn first_last(s: String) -> i8 {
    let a = s.chars().nth(0).unwrap();
    let b = s.chars().last().unwrap();
    let a_b: String = vec![a, b].into_iter().collect();
    return a_b.parse::<i8>().unwrap();
}

fn part_1() {
    let file = File::open("./calibration.txt").expect("File does not exist");
    let lines: Result<Vec<_>, std::io::Error> = BufReader::new(file).lines().into_iter().collect();

    if let Ok(ls) = lines {
        let value = ls
            .into_iter()
            .map(clean_str)
            .map(first_last)
            .fold(0, |acc, i| acc + i as i32);
        println!("part 1: {}", value);
    } else {
        println!("Error");
    }
}

fn part_2() {
    let file = File::open("./calibration.txt").expect("File does not exist");
    let lines: Result<Vec<_>, std::io::Error> = BufReader::new(file).lines().into_iter().collect();

    if let Ok(ls) = lines {
        let value = ls
            .into_iter()
            .map(|s| clean_str(replace_words(s)))
            .map(first_last)
            .fold(0, |acc, i| acc + i as i32);
        println!("part 2: {}", value);
    } else {
        println!("Error");
    }
}

fn main() {
    part_1();
    part_2();
}

#[cfg(test)]
mod tests {
    use crate::{clean_str, first_last, replace_words};

    #[test]
    fn test_clean_str() {
        let test_strs: Vec<String> = vec![
            "1abc2".to_owned(),
            "pqr3stu8vwx".to_owned(),
            "a1b2c3d4e5f".to_owned(),
            "treb7uchet".to_owned(),
        ];
        let values: i32 = test_strs
            .into_iter()
            .map(clean_str)
            .map(first_last)
            .fold(0, |acc, i| acc + i as i32);
        println!("{}", values);
    }

    #[test]
    fn test_replace_words() {
        let test_strs: Vec<String> = vec![
            "two1nine".to_owned(),
            "eightwothree".to_owned(),
            "abcone2threexyz".to_owned(),
            "xtwone3four".to_owned(),
            "4nineeightseven2".to_owned(),
            "zoneight234".to_owned(),
            "7pqrstsixteen".to_owned(),
        ];

        for t in test_strs {
            println!("{}", first_last(clean_str(replace_words(t))));
        }
    }
}
