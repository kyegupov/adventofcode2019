use std::env;
use std::fs;
use std::error::Error;
use std::collections::{BTreeMap, BTreeSet, VecDeque, HashMap};
#[derive(Debug)]
struct Vm {
    a: Vec<i64>,
    i: usize,
    relbase: usize,
    halt: bool,
}

fn distances_accessible(board: &Vec<Vec<char>>, x: usize, y: usize, has_keys: &BTreeSet<char>) -> Vec<(char, (usize, usize), i64)> {
    let sides: Vec<(i64, i64)> = vec![(0,-1),(1,0),(0,1),(-1,0)];
    let mut queue = VecDeque::new();
    let mut dist = BTreeMap::new();
    dist.insert((x,y),0i64);
    queue.push_back((x,y));
    let mut res = vec![];
    while queue.len() > 0 {
        let (x, y) = queue.pop_front().unwrap();
        let d = dist[&(x, y)]+1;
        for (dx,dy) in &sides {
            let xx = (x as i64 + dx) as usize;
            let yy = (y as i64 + dy) as usize;
            if !dist.contains_key(&(xx, yy)) {
                let c = board[yy][xx];
                if c == '.' || c == '@' || (c>='a' && c <='z') || (c>='A' && c <= 'Z' && has_keys.contains(&c.to_ascii_lowercase())) {
                    dist.insert((xx, yy), d);
                    if  c>='a' && c <='z' && !has_keys.contains(&c) {
                        res.push((c, (xx,yy), d));
                    } else {
                        queue.push_back((xx, yy));
                    }
                }
            }
        }
    }
    // dbg!(&res);
    res
}

fn distances_letters(board: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<(char, i64)> {
    let sides: Vec<(i64, i64)> = vec![(0,-1),(1,0),(0,1),(-1,0)];
    let mut queue = VecDeque::new();
    let mut dist = BTreeMap::new();
    dist.insert((x,y),0i64);
    queue.push_back((x,y));
    let mut res = vec![];
    while queue.len() > 0 {
        let (x, y) = queue.pop_front().unwrap();
        for (dx,dy) in &sides {
            let xx = (x as i64 + dx) as usize;
            let yy = (y as i64 + dy) as usize;
            if !dist.contains_key(&(xx, yy)) {
                let c = board[yy][xx];
                if c != '#' {
                    let d = dist[&(x, y)]+1;
                    dist.insert((xx, yy), d);
                    if (c>='a' && c <='z') || (c>='A' && c <='Z') {
                        res.push((c, d));
                    }
                    if !(c>='A' && c <='Z') {
                        queue.push_back((xx, yy));
                    }
                }
            }
        }
    }
    // dbg!(&res);
    res
}

fn solve(board: &Vec<Vec<char>>, x: usize, y: usize, has_keys: &mut BTreeSet<char>, all_keys_num: usize, key_order: &mut Vec<char>,
    current_steps: i64, cache: &mut BTreeMap<(usize, usize, BTreeSet<char>), Vec<(char, (usize, usize), i64)>>,
    cache2: &mut BTreeMap<(usize, usize, BTreeSet<char>), i64>,
) -> i64 {
    if let Some(solution) = cache2.get(&(x, y, has_keys.clone())) {
        return current_steps + *solution;
    }
    let mut best_solution = 99999999;
    let dists =
        cache.entry((x, y, has_keys.clone())).or_insert_with(||distances_accessible(&board, x, y, &has_keys)).clone();
    // dists.sort_by_key(|(c, xy, d)|*d);
    // dbg!(&dists);
    for (key, (xx,yy), d) in dists.iter() {
        key_order.push(*key);
        let solution = if key_order.len() == all_keys_num {
            d + current_steps
        } else {
            has_keys.insert(*key);
            let res = solve(&board, *xx, *yy, has_keys, all_keys_num, key_order, current_steps+d, cache, cache2);
            has_keys.remove(key);
            res
        };
        if best_solution > solution {
            best_solution = solution;
            // dbg!(has_keys.len());
            // dbg!(best_solution);
            // dbg!(cache.len());
            // let mut s = String::new();
            // for c in key_order.iter() {
            //     s.push(*c);
            // }
            // println!("{}", s);
        }
        key_order.pop();
    }
    cache2.insert((x, y, has_keys.clone()), best_solution-current_steps);
    best_solution
}


fn solve2(all_distances: &BTreeMap<char, Vec<(char, i64)>>, orig_mark: char, has_keys: &mut BTreeSet<char>, all_keys_num: usize,
    visited_marks: &mut Vec<char>, current_steps: i64, prev_best_solution: i64, cache: &mut BTreeMap<(char, BTreeSet<char>), Vec<(char, i64)>>,
) -> i64 {
    let mut best_solution = prev_best_solution;

    let ckey = (orig_mark, has_keys.clone());
    let dists = cache.entry(ckey).or_insert_with(||{
        let mut accessible_keys = BTreeMap::new();
        let mut doors = BTreeSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((orig_mark, 0));
        while !queue.is_empty() {
            let (m0, d0) = queue.pop_front().unwrap();
            if current_steps + d0 > best_solution {
                break;
            }

            for (m,d) in &all_distances[&m0] {
                if *m>='a' && *m <='z' && !has_keys.contains(m) {
                    if *accessible_keys.get(m).unwrap_or(&9999999) > d+d0 {
                        accessible_keys.insert(*m, d+d0);
                    }
                } else {
                    if has_keys.contains(&m.to_ascii_lowercase()) {
                        if !doors.contains(m) {
                            queue.push_back((*m, d0+d));
                        }
                    }
                    doors.insert(m);
                }
            }
        }
        let mut dists: Vec<(char, i64)> = accessible_keys.into_iter().collect();
        dists.sort_by_key(|(c, d)|*d);
        return dists
    }).clone();
    for (key, d) in dists.iter() {
        if current_steps + d > best_solution {
            break;
        }
        visited_marks.push(*key);
        let solution = if has_keys.len() == all_keys_num {
            d + current_steps
        } else {
            solve2(all_distances, *key, has_keys, all_keys_num, visited_marks, current_steps+d, best_solution, cache)
        };
        if best_solution > solution {
            best_solution = solution;
            dbg!(has_keys.len());
            dbg!(best_solution);
            let mut s = String::new();
            for c in visited_marks.iter() {
                s.push(*c);
            }
            println!("{}", s);
        }
        has_keys.remove(key);
        visited_marks.pop();
    }
    best_solution
}

fn main() -> Result<(), Box<dyn Error>> {
    let task_part = &env::args().collect::<Vec<String>>()[1];
    let input = fs::read_to_string("input.txt")?;

    let board: Vec<Vec<char>> = input.lines().map(|l|l.chars().collect()).collect();

    let mut x = 0;
    let mut y = 0;
    let mut all_keys = BTreeSet::new();
    let mut all_distances: BTreeMap<char, Vec<(char, i64)>> = BTreeMap::new();

    for (yy, row) in board.iter().enumerate() {
        for (xx, c) in row.iter().enumerate() {
            if *c == '@' {
                x = xx;
                y = yy;
                // all_distances.insert(*c, distances_letters(&board, xx, yy));
            }
            if *c>='a' && *c <='z' {
                all_keys.insert(*c);
                // all_distances.insert(*c, distances_letters(&board, xx, yy));
            }
            if *c>='A' && *c <='Z' {
                // all_distances.insert(*c, distances_letters(&board, xx, yy));
            }
        }
    }

    let mut has_keys = BTreeSet::new();
    let mut visited_marks = vec![];
    let mut cache = BTreeMap::new();
    let mut cache2 = BTreeMap::new();
    dbg!(solve(&board, x, y, &mut has_keys, all_keys.len(), &mut visited_marks, 0, &mut cache, &mut cache2));
    dbg!(cache.len());
    dbg!(cache2.len());

    Ok(())
}
