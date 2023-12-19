use std::cmp::{max, min, Ordering};
use std::collections::*;
use std::fs::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i64,
    xy: (i64, i64),
    dir: Dir,
    streak: i64,
}

impl Ord for State {
    fn cmp(&self, &other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
        // .then_with(|| self.streak.cmp(&other.streak))
        // .then_with(|| (self.xy.0 + self.xy.1).cmp(&(other.xy.0 + other.xy.1)))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(lns: &mut Vec<Vec<i64>>) -> i64 {
    return shortest_path(lns, 3, next_dir);
}

fn part2(lns: &Vec<Vec<i64>>) -> i64 {
    return shortest_path(lns, 10, next_dir2);
}

fn dir_coords(d: Dir) -> (i64, i64) {
    match d {
        Dir::Left => (-1, 0),
        Dir::Right => (1, 0),
        Dir::Up => (0, -1),
        Dir::Down => (0, 1),
    }
}

fn move_point(xy: (i64, i64), d: Dir) -> (i64, i64) {
    let dc = dir_coords(d);
    return (xy.0 + dc.0, xy.1 + dc.1);
}

fn next_dir(d: Dir, s: i64) -> Vec<(Dir, i64)> {
    match d {
        Dir::Left => vec![(Dir::Left, s + 1), (Dir::Up, 1), (Dir::Down, 1)],
        Dir::Right => vec![(Dir::Right, s + 1), (Dir::Up, 1), (Dir::Down, 1)],
        Dir::Up => vec![(Dir::Up, s + 1), (Dir::Left, 1), (Dir::Right, 1)],
        Dir::Down => vec![(Dir::Down, s + 1), (Dir::Left, 1), (Dir::Right, 1)],
    }
}

fn next_dir2(d: Dir, s: i64) -> Vec<(Dir, i64)> {
    if s < 4 {
        vec![(d, s + 1)]
    } else {
        match d {
            Dir::Left => vec![(Dir::Left, s + 1), (Dir::Up, 1), (Dir::Down, 1)],
            Dir::Right => vec![(Dir::Right, s + 1), (Dir::Up, 1), (Dir::Down, 1)],
            Dir::Up => vec![(Dir::Up, s + 1), (Dir::Left, 1), (Dir::Right, 1)],
            Dir::Down => vec![(Dir::Down, s + 1), (Dir::Left, 1), (Dir::Right, 1)],
        }
    }
}

fn shortest_path(
    cs: &Vec<Vec<i64>>,
    max_streak: i64,
    next_fn: fn(Dir, i64) -> Vec<(Dir, i64)>,
) -> i64 {
    let goal = (cs[0].len() as i64 - 1, cs.len() as i64 - 1);
    //let goal = (4, 0);
    let mut vs: HashMap<((i64, i64), Dir, i64), i64> = HashMap::new();
    let mut q: BinaryHeap<State> = BinaryHeap::new();

    let (max_x, max_y) = (cs[0].len() as i64 - 1, cs.len() as i64 - 1);

    q.push(State {
        cost: cs[0][1],
        xy: (1, 0),
        dir: Dir::Right,
        streak: 1,
    });

    q.push(State {
        cost: cs[1][0],
        xy: (0, 1),
        dir: Dir::Down,
        streak: 1,
    });

    while let Some(State {
        cost,
        xy,
        dir,
        streak,
    }) = q.pop()
    {
        if xy == goal {
            return cost;
        }

        for (nd, ns) in next_fn(dir, streak).iter() {
            let nxy = move_point(xy, *nd);
            if 0 <= nxy.0 && nxy.0 <= max_x && 0 <= nxy.1 && nxy.1 <= max_y {
                let ncost = cost + cs[nxy.1 as usize][nxy.0 as usize];

                if *ns < max_streak + 1 {
                    let ccost = *vs
                        .get(&((nxy.0, nxy.1), *nd, *ns))
                        .or(Some(&i64::MAX))
                        .unwrap();

                    if ncost < ccost {
                        vs.insert((nxy, *nd, *ns), ncost);
                        q.push(State {
                            cost: ncost,
                            xy: nxy,
                            dir: *nd,
                            streak: *ns,
                        });
                    }
                }
            }
        }
    }
    return 0;
}

fn main() {
    let input = read_to_string("./input.txt").expect("File does not exist");
    let mut lns: Vec<Vec<i64>> = input
        .trim()
        .split("\n")
        .map(|cs| {
            cs.chars()
                .map(|c| i64::from(c.to_digit(10).unwrap()))
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    println!("part 1: {}", part1(&mut lns));
    println!("part 2: {}", part2(&mut lns));
}
