use std::env;
use std::fs;
use std::error::Error;
use std::collections::BTreeMap;

fn sum_kids(l: &BTreeMap<&str, Vec<&str>>, key: &str) -> usize {

    let r = l.get(key).unwrap_or(&vec![]).iter().map(|x|1+sum_kids(l,x)).sum();
    // dbg!(key, r);
    r
}

fn main() -> Result<(), Box<dyn Error>> {
    let task_part = &env::args().collect::<Vec<String>>()[1];
    let input = fs::read_to_string("input.txt")?;
    let mut res: usize = 0;

    if task_part=="1" {
        let lines: Vec<&str> = input.lines().collect();
        let mut links: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        for line in lines {
            if let [a, b] = line.split(')').collect::<Vec<&str>>()[..] {
                links.entry(a).or_insert(vec![]).push(b);
            }
        }
        res = links.keys().map(|k|sum_kids(&links, k)).sum();
    }

    if task_part=="2" {
        let lines: Vec<&str> = input.lines().collect();
        let mut dad: BTreeMap<&str, &str> = BTreeMap::new();
        for line in lines {
            if let [a, b] = line.split(')').collect::<Vec<&str>>()[..] {
                dad.insert(b, a);
            }
        }
        let mut you_pos = dad["YOU"];
        let mut you_steps = 0;
        while you_pos != "" {
            let mut san_pos = dad["SAN"];
            let mut san_steps = 0;
            while san_pos != "" && san_pos != you_pos {
                san_pos = dad.get(san_pos).unwrap_or(&"");
                // dbg!(san_pos);
                san_steps += 1;
            }
            if san_pos == you_pos {
                res = you_steps + san_steps;
                break;
            }
            you_pos = dad.get(you_pos).unwrap_or(&"");
            you_steps += 1;
        }
    }

    println!("{}", res);
    Ok(())
}
