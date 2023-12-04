use std::fmt::Display;
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

fn main() {
    let file = File::open("./schematic.txt").expect("File does not exist");
    let lines: Result<Vec<_>, std::io::Error> = BufReader::new(file).lines().into_iter().collect();

    if let Ok(lns) = lines {
        let adj: Vec<Vec<u32>> = find_adjacent(lns);

        let mut sum: u32 = 0;
        let mut prod: u32 = 0;

        for v in adj.iter() {
            sum += v.iter().sum::<u32>();
            if v.len() == 2 {
                prod += v.iter().product::<u32>()
            }
        }

        println!("part_1: {}\npart_2: {}", sum, prod);
    }
}

fn find_adjacent(lines: Vec<String>) -> Vec<Vec<u32>> {
    let markers: Vec<Vec<usize>> = lines.iter().map(|s| find_marker(&s)).collect();
    let parts: Vec<Vec<PartNum>> = lines.iter().map(|s| find_range(&s)).collect();

    let mut adj: Vec<Vec<u32>> = Vec::new();

    for (i, ms) in markers.into_iter().enumerate() {
        for m in ms.iter() {
            let mut madj: Vec<u32> = Vec::new();

            if i > 0 {
                for p in parts[i - 1].iter() {
                    if p.start == m - 1 || p.start == *m || p.start == m + 1 {
                        madj.push(p.part_num);
                    } else if p.end == m - 1 || p.end == *m || p.end == m + 1 {
                        madj.push(p.part_num);
                    }
                }
            }

            for p in parts[i].iter() {
                if p.start == m + 1 || p.end == m - 1 {
                    madj.push(p.part_num);
                }
            }

            if i < lines.len() {
                for p in parts[i + 1].iter() {
                    if p.start == m - 1 || p.start == *m || p.start == m + 1 {
                        madj.push(p.part_num);
                    } else if p.end == m - 1 || p.end == *m || p.end == m + 1 {
                        madj.push(p.part_num);
                    }
                }
            }

            adj.push(madj);
        }
    }
    return adj;
}

#[warn(dead_code)]
fn find_marker(s: &str) -> Vec<usize> {
    let mut acc: Vec<usize> = Vec::from([]);
    for (i, c) in s.chars().enumerate() {
        if c != '.' && !c.is_digit(10) {
            acc.push(i);
        }
    }
    return acc;
}

#[derive(Clone, Debug, PartialEq)]
struct PartNum {
    part_num: u32,
    start: usize,
    end: usize,
}

impl Display for PartNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PartNum ( part_num: {}, start: {}, end: {} )",
            self.part_num, self.start, self.end
        )
    }
}

#[warn(dead_code)]
fn find_range(s: &str) -> Vec<PartNum> {
    let mut acc: Vec<PartNum> = Vec::from([]);
    let mut start: usize = 0;

    while start < s.len() {
        if s.chars().nth(start).unwrap().is_digit(10) {
            let mut end = start + 1;

            while end < s.len() && s.chars().nth(end).unwrap().is_digit(10) {
                end += 1;
            }
            let pn = PartNum {
                part_num: s[start..end].parse::<u32>().unwrap(),
                start,
                end: end - 1,
            };
            acc.push(pn);
            start = end;
        }
        start += 1;
    }
    return acc;
}

#[cfg(test)]
mod tests {
    use crate::find_range;
    use crate::PartNum;

    #[test]
    fn test_parse_schematic() {
        let schematic = vec![
            String::from("467..114.."),
            String::from("...*......"),
            String::from("..35..633."),
            String::from("......#..."),
            String::from("617*......"),
            String::from(".....+.58."),
            String::from("..592....."),
            String::from("......755."),
            String::from("...$.*...."),
            String::from(".664.598.."),
        ];

        let s: Vec<Vec<PartNum>> = schematic.iter().map(|s| find_range(&s)).collect();
        println!("{:?}", s[2]);
    }
}
