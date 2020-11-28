use std::env;
use std::fs;
use std::error::Error;
use std::collections::{BTreeSet,BTreeMap};

fn main() -> Result<(), Box<dyn Error>> {
    let task_part = &env::args().collect::<Vec<String>>()[1];
    let input = fs::read_to_string("input.txt")?;

    let mut res = 0;
    if task_part=="1" {
        let rows: Vec<Vec<&str>> = input.lines().map(|r|r.split(',').collect()).collect();
        let mut wires: Vec<BTreeSet<(i32,i32)>> = vec![];
        for r in rows {
            let mut xy = (0, 0);
            let mut set = BTreeSet::new();
            for code in r {
                let delta = match code.chars().next().unwrap() {
                    'U'=> (0,1),
                    'D'=> (0,-1),
                    'L'=> (-1, 0),
                    'R'=> (1, 0),
                    _ => panic!(),
                };
                for i in 0..code[1..].parse::<i32>()? {
                    xy = (xy.0+delta.0,xy.1+delta.1);
                    set.insert(xy);
                    // println!("{:?}", xy);
                };
            }
            wires.push(set);
        }
        // println!("{:?}", wires);
        // println!("{:?}", wires[0].intersection(&wires[1]).collect::<Vec<_>>());
        res = wires[0].intersection(&wires[1]).map(|xy|xy.0.abs()+xy.1.abs()).min().unwrap();

    }

    if task_part=="2" {
        let rows: Vec<Vec<&str>> = input.lines().map(|r|r.split(',').collect()).collect();

        let mut wires: Vec<BTreeMap<(i32,i32), usize>> = vec![];
        for r in rows {
            let mut xy = (0, 0);
            let mut set = BTreeMap::new();
            let mut steps = 0;
            for code in r {
                let delta = match code.chars().next().unwrap() {
                    'U'=> (0,1),
                    'D'=> (0,-1),
                    'L'=> (-1, 0),
                    'R'=> (1, 0),
                    _ => panic!(),
                };
                for i in 0..code[1..].parse::<i32>()? {
                    steps += 1;
                    xy = (xy.0+delta.0,xy.1+delta.1);
                    set.insert(xy,steps);
                    // println!("{:?}", xy);
                };
            }
            wires.push(set);
        }
        // println!("{:?}", wires);
        // println!("{:?}", wires[0].intersection(&wires[1]).collect::<Vec<_>>());
        res = wires[0].keys().filter(|k|wires[1].contains_key(k)).map(|xy|wires[0][xy]+wires[1][xy]).min().unwrap() as i32;
    }


    println!("{}", res);
    Ok(())
}
