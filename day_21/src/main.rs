use std::collections::*;
use std::fs::*;

fn main() {
    let input = read_to_string("input.txt").expect("File does not exist");
    let map: Vec<Vec<char>> = input
        .trim()
        .split("\n")
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    println!("part 1: {}", part1(&map));
    println!("part 2: {}", part2(&map));
}

fn part1<'a>(map: &Vec<Vec<char>>) -> i64 {
    let start = find_start(map);
    return search_map(map, start, 64);
}

fn part2(map: &Vec<Vec<char>>) -> i64 {
    let start = find_start(map);
    for i in vec![65, 196, 327].iter() {
        println!("{}: {}", i, search_map(map, start, *i));
    }
    return 0;
}

fn find_start(map: &Vec<Vec<char>>) -> (i64, i64) {
    for r in 0..map.len() - 1 {
        for c in 0..map[0].len() - 1 {
            if map[r][c] == 'S' {
                return (c as i64, r as i64);
            }
        }
    }

    return (0, 0);
}

fn search_map(map: &Vec<Vec<char>>, s: (i64, i64), max_steps: i64) -> i64 {
    let mut plots: HashSet<(i64, i64, i64)> = HashSet::new();
    let mut ps: HashSet<(i64, i64, i64)> = HashSet::new();
    let mut q: VecDeque<(i64, i64, i64)> = VecDeque::new();

    let xmax = map[0].len() as i64;
    let ymax = map.len() as i64;

    let nsteps: [(i64, i64); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    q.push_front((s.0, s.1, 0));

    while let Some((x, y, steps)) = q.pop_front() {
        let ns: Vec<(i64, i64)> = nsteps.iter().map(|(nx, ny)| (x + nx, y + ny)).collect();

        for (nx, ny) in ns {
            let mut nnx = nx % xmax;
            let mut nny = ny % ymax;

            if nnx < 0 {
                nnx = nnx + xmax;
            }
            if nny < 0 {
                nny = nny + ymax;
            }

            if steps + 1 < max_steps {
                if "S.".contains(map[nnx as usize][nny as usize])
                    && !plots.contains(&(nx, ny, steps + 1))
                {
                    plots.insert((nx, ny, steps + 1));
                    q.push_back((nx, ny, steps + 1));
                }
            } else if "S.".contains(map[nnx as usize][nny as usize]) {
                ps.insert((nx, ny, steps + 1));
            }
        }
    }

    return ps.len() as i64; //total;
}
