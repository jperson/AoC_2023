use num::integer::lcm;
use std::collections::*;
use std::fs::*;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Type {
    FlipFlop(bool, Vec<String>),
    Conjunction(HashMap<String, bool>, Vec<String>),
    Broadcast(Vec<String>),
}

fn main() {
    let input = read_to_string("input.txt").expect("File does not exist");
    let lns: Vec<&str> = input.trim().split("\n").collect();

    println!("part 1: {}", part1(&lns));
    println!("part 2: {}", part2(&lns));
}

fn part1(lns: &Vec<&str>) -> i64 {
    let mut m: HashMap<String, Type> = parse_input(lns);
    let mut total_lows: i64 = 0;
    let mut total_highs: i64 = 0;

    for _ in 0..1000 {
        let (l, h) = press_button(&mut m);
        total_lows += l;
        total_highs += h;
    }

    return total_lows * total_highs;
}

fn part2(lns: &Vec<&str>) -> i64 {
    let mut m: HashMap<String, Type> = parse_input(lns);

    let mut cyc: HashMap<String, usize> = m
        .values()
        .find_map(|v| {
            if let Type::Conjunction(inp, out) = v {
                if out.contains(&String::from("rx")) {
                    return Some(inp.keys().map(|k| (k.clone(), 0)));
                }
            }
            None
        })
        .unwrap()
        .collect();

    for i in 1.. {
        let mut q: VecDeque<(String, bool)> = VecDeque::new();
        q.push_back(("broadcaster".to_string(), false));
        while let Some((action, signal)) = q.pop_front() {
            let (s, ts) = send_pulse(action.clone(), signal, &mut m);
            for t in &ts {
                q.push_back((t.clone(), s));
                if !s && cyc.get(t) == Some(&0) {
                    cyc.insert(t.to_string(), i);
                    if cyc.values().all(|v| *v > 0) {
                        return cyc.values().fold(1, |acc, c| lcm(acc, *c)) as i64;
                    }
                }
            }
        }
    }

    return 0;
}

fn send_pulse(action: String, signal: bool, m: &mut HashMap<String, Type>) -> (bool, Vec<String>) {
    if !m.contains_key(&action) {
        return (signal, vec![]);
    }

    let tp = m.get(&action).unwrap().to_owned();
    match tp {
        Type::FlipFlop(state, targets) if !signal => {
            let fsignal = !state;
            m.insert(action.clone(), Type::FlipFlop(fsignal, targets.clone()));
            for t in &targets {
                if let Type::Conjunction(im, _) = m.get_mut(t).unwrap() {
                    im.insert(action.clone(), fsignal);
                }
            }
            return (!state, targets.clone());
        }
        Type::Conjunction(im, targets) => {
            let all: bool = im.values().all(|s| *s);
            for t in &targets {
                if let Some(Type::Conjunction(im, _)) = m.get_mut(t) {
                    im.insert(action.clone(), !all);
                }
            }
            return (!im.values().all(|s| *s), targets.clone());
        }
        Type::Broadcast(targets) => {
            for t in &targets {
                if let Some(Type::Conjunction(im, _)) = m.get_mut(t) {
                    im.insert(action.clone(), signal);
                }
            }
            return (signal, targets.clone());
        }
        _ => return (signal, vec![]),
    }
}

fn press_button(m: &mut HashMap<String, Type>) -> (i64, i64) {
    let mut q: VecDeque<(String, bool)> = VecDeque::new();
    let mut lows: i64 = 0;
    let mut highs: i64 = 0;

    q.push_back(("broadcaster".to_string(), false));

    while let Some((action, signal)) = q.pop_front() {
        if signal {
            highs += 1
        } else {
            lows += 1
        }
        let (s, ts) = send_pulse(action.clone(), signal, m);
        for t in ts {
            q.push_back((t, s));
        }
    }

    return (lows, highs);
}

fn parse_input(lns: &Vec<&str>) -> HashMap<String, Type> {
    let mut m: HashMap<String, Type> = HashMap::new();
    for l in lns.iter() {
        let sps = l.split(" -> ").collect::<Vec<&str>>();
        let n = sps[0];
        let targets: Vec<String> = sps[1]
            .split(", ")
            .map(|s| s.chars().collect::<String>())
            .collect::<Vec<String>>();

        if n.starts_with("%") {
            m.insert(n[1..].to_owned(), Type::FlipFlop(false, targets));
        } else if n.starts_with("&") {
            m.insert(
                n[1..].to_owned(),
                Type::Conjunction(HashMap::new(), targets),
            );
        } else if n == "broadcaster" {
            m.insert(n.to_owned(), Type::Broadcast(targets));
        }
    }

    let mut inputs: HashMap<String, HashMap<String, bool>> = HashMap::new();
    for (k, t) in &m {
        let out = match t {
            Type::FlipFlop(_, o) | Type::Conjunction(_, o) | Type::Broadcast(o) => o.clone(),
        };
        for o in out {
            inputs.entry(o).or_default().insert(k.clone(), false);
        }
    }

    for (k, t) in &mut m {
        if let Type::Conjunction(im, _) = t {
            *im = inputs.remove(k).unwrap_or_default();
        }
    }

    return m;
}
