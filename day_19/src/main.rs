use num::abs;
//use std::cmp::{max, min, Ordering};
use std::collections::*;
use std::fs::*;
use std::iter::zip;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Part {
    a: i64,
    x: i64,
    m: i64,
    s: i64,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Op {
    LT,
    GT,
    A,
    R,
}

#[derive(Debug, Eq, PartialEq)]
struct Workflow {
    name: String,
    rules: Vec<(String, (Op, i64, String))>,
    last: String,
}

fn parse_rules(r: &str) -> (Vec<(String, (Op, i64, String))>, String) {
    let mut rm: Vec<(String, (Op, i64, String))> = Vec::new();

    let parts = r.split(",").collect::<Vec<&str>>();
    for p in parts[0..parts.len() - 1].iter() {
        let sps: Vec<&str> = p.split(":").collect();
        let nwf = sps[1].to_owned();

        if sps[0].contains(">") {
            let nv = sps[0].split(">").collect::<Vec<&str>>()[0].to_owned();
            let vl = sps[0].split(">").collect::<Vec<&str>>()[1]
                .parse::<i64>()
                .unwrap();
            rm.push((nv, (Op::GT, vl, nwf)));
        } else if sps[0].contains("<") {
            let nv = sps[0].split("<").collect::<Vec<&str>>()[0].to_owned();
            let vl = sps[0].split("<").collect::<Vec<&str>>()[1]
                .parse::<i64>()
                .unwrap();
            rm.push((nv, (Op::LT, vl, nwf)));
        }
    }

    return (rm, parts[parts.len() - 1].to_owned());
}

fn parse_part(p: &str) -> Part {
    let mut part: Part = Part {
        x: 0,
        m: 0,
        a: 0,
        s: 0,
    };

    let ratings: Vec<&str> = p[1..p.len() - 1].split(",").collect();
    for r in ratings.iter() {
        let vs: Vec<&str> = r.split("=").collect();
        let (name, value) = (vs[0], vs[1]);
        match name {
            "x" => part.x = value.parse::<i64>().unwrap(),
            "m" => part.m = value.parse::<i64>().unwrap(),
            "a" => part.a = value.parse::<i64>().unwrap(),
            "s" => part.s = value.parse::<i64>().unwrap(),
            _ => (),
        }
    }
    return part;
}

fn parse_workflows(ws: &Vec<&str>) -> (String, HashMap<String, Workflow>) {
    let mut pwfs: HashMap<String, Workflow> = HashMap::new();

    for w in ws.iter() {
        let wsps = w.split("{").collect::<Vec<&str>>();
        let name: String = wsps[0].to_owned();
        let rules: &str = &wsps[1][0..wsps[1].len() - 1];
        let (rules, last) = parse_rules(rules);
        let w = Workflow {
            name: wsps[0].to_owned(),
            rules,
            last,
        };
        pwfs.insert(name.clone(), w);
    }
    pwfs.insert(
        "A".to_owned(),
        Workflow {
            name: "A".to_owned(),
            rules: vec![],
            last: "A".to_owned(),
        },
    );

    pwfs.insert(
        "R".to_owned(),
        Workflow {
            name: "R".to_owned(),
            rules: vec![],
            last: "R".to_owned(),
        },
    );

    return ("in".to_owned(), pwfs);
}

fn part_value(s: &str, p: Part) -> i64 {
    if s == "a" {
        return p.a;
    }
    if s == "x" {
        return p.x;
    }
    if s == "m" {
        return p.m;
    }
    return p.s;
}

fn pr_idx(s: &str) -> usize {
    if s == "a" {
        return 0;
    }
    if s == "x" {
        return 1;
    }
    if s == "m" {
        return 2;
    }
    return 3;
}

fn op_idx(op: &Op) -> usize {
    if *op == Op::GT {
        return 0;
    }
    return 1;
}

fn fnrs(
    wf: &HashMap<String, Workflow>,
    vrs: &mut Vec<[[i64; 2]; 4]>,
    cur: String,
    rs: [[i64; 2]; 4],
) -> () {
    let w = wf.get(&cur).unwrap();
    let mut nrs = rs.clone();

    if w.name == "A" {
        vrs.push(nrs.clone());
        return;
    } else if w.rules.len() == 0 && w.name != "R" {
        fnrs(wf, vrs, w.last.clone(), nrs.clone());
        return;
    } else if w.name == "R" {
        return;
    }

    for (pr, (op, vl, nwf)) in w.rules.iter() {
        let pid = pr_idx(pr.as_str());
        let oid = op_idx(op);
        let prv = nrs[pid][oid];

        nrs[pid][oid] = if oid == 0 { *vl + 1 } else { *vl };
        fnrs(wf, vrs, nwf.clone(), nrs);

        nrs[pid][oid] = prv;
        nrs[pid][oid ^ 1] = if oid == 0 { *vl + 1 } else { *vl };
    }
    fnrs(wf, vrs, w.last.clone(), nrs.clone());
}

fn run1(p: Part, mut s: String, wf: &HashMap<String, Workflow>) -> String {
    while let Some(w) = wf.get(&s) {
        if w.rules.len() == 0 {
            return w.last.clone();
        }
        'found: {
            for (nm, (op, vl, nwf)) in w.rules.iter() {
                let pv: i64 = part_value(nm.as_str(), p);
                if (*op == Op::LT && pv < *vl) || (*op == Op::GT && pv > *vl) {
                    s = nwf.clone();
                    break 'found;
                }
            }
            s = w.last.clone();
        }
    }

    return s;
}

fn part1(lns: &Vec<&str>) -> i64 {
    let workflows: Vec<&str> = lns[0].split("\n").collect();
    let parts: Vec<&str> = lns[1].split("\n").collect();
    let parts = parts.iter().map(|p| parse_part(p)).collect::<Vec<Part>>();
    let (start, wfs) = parse_workflows(&workflows);

    let mut total: i64 = 0;
    for p in parts.iter() {
        if run1(*p, start.clone(), &wfs) == "A" {
            total += p.a + p.s + p.x + p.m;
        }
    }
    return total;
}

fn part2(lns: &Vec<&str>) -> i64 {
    let workflows: Vec<&str> = lns[0].split("\n").collect();
    let (_, wfs) = parse_workflows(&workflows);

    let mut vrs: Vec<[[i64; 2]; 4]> = Vec::new();
    let rs: [[i64; 2]; 4] = [[1, 4001]; 4];

    fnrs(&wfs, &mut vrs, "in".to_owned(), rs);
    let mut total: i64 = 0;

    for v in vrs.iter() {
        total += v.iter().fold(1, |acc, [a, b]| acc * (b - a));
    }
    return total;
}

fn main() {
    let input = read_to_string("./input.txt").expect("File does not exist");
    let lns: Vec<&str> = input.trim().split("\n\n").collect();

    println!("part 1: {}", part1(&lns));
    println!("part 2: {}", part2(&lns));
}
