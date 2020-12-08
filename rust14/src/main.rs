use std::{env, vec};
use std::fs;
use std::error::Error;
use std::collections::{BTreeMap, BTreeSet};

// fn get_ore(name: &str, qty: i64, formulas: &BTreeMap<String, (i64, Vec<(String, i64)>)>) -> i64 {
//     let res = ingrs.iter().map(|(iname, iqty)|if iname=="ORE" {*iqty*(qty/fqty)} else {get_ore(iname, *iqty*(qty/fqty), formulas)}).sum();
//     dbg!(name, qty, res);
//     res
// }

fn main() -> Result<(), Box<dyn Error>> {
    let task_part = &env::args().collect::<Vec<String>>()[1];
    let input = fs::read_to_string("input.txt")?;

    let mut res = 0;

    let parse_pair = |x: &str| {
        let mut parts = x.trim().split(" ");
        let qty = parts.next().unwrap().parse::<i64>().unwrap();
        let name = parts.next().unwrap().to_owned();
        return (name, qty);
    };

    if task_part=="1" {
        let mut formulas = BTreeMap::new();
        let mut required_by = BTreeMap::new();
        for line in input.lines() {
            let mut parts = line.split("=>");
            let inps = parts.next().unwrap();
            let out = parts.next().unwrap();
            let (name, qty) = parse_pair(out);
            let ingrs = inps.split(", ").map(parse_pair).collect::<Vec<_>>();
            for ing in ingrs.iter() {
                required_by.entry(ing.0.to_owned()).or_insert(BTreeSet::new()).insert(name.to_owned());
            }
            formulas.insert(name, (qty, ingrs));
        }

        let mut toposorted = vec!["FUEL".to_owned()];
        while !required_by.is_empty() {
            dbg!(&required_by);
            let to_pop = required_by.iter().filter(|(kid,dads)|!dads.iter().any(|x|required_by.contains_key(x))).map(|(k,_v)|k.to_owned()).collect::<Vec<_>>();
            for k in to_pop {
                required_by.remove(&k);
                toposorted.push(k);
            }
        }
        dbg!(&toposorted);
        assert!(toposorted.pop().unwrap()=="ORE");

        let mut need: BTreeMap<String, i64> = BTreeMap::new();
        need.insert("FUEL".to_owned(), 1);
        for key in toposorted {
            if need.contains_key(&key) {
                let (fqty, ingrs) = &formulas[&key];
                let qty = need[&key];
                for (iname, iq) in ingrs {
                    *need.entry(iname.to_owned()).or_insert(0i64) += iq * ((qty+fqty-1)/fqty);
                }
            }
        }
        dbg!(&need);
        res = need["ORE"];
    }

    if task_part=="2" {
        let mut formulas = BTreeMap::new();
        let mut required_by = BTreeMap::new();
        for line in input.lines() {
            let mut parts = line.split("=>");
            let inps = parts.next().unwrap();
            let out = parts.next().unwrap();
            let (name, qty) = parse_pair(out);
            let ingrs = inps.split(", ").map(parse_pair).collect::<Vec<_>>();
            for ing in ingrs.iter() {
                required_by.entry(ing.0.to_owned()).or_insert(BTreeSet::new()).insert(name.to_owned());
            }
            formulas.insert(name, (qty, ingrs));
        }

        let mut toposorted = vec!["FUEL".to_owned()];
        while !required_by.is_empty() {
            dbg!(&required_by);
            let to_pop = required_by.iter().filter(|(kid,dads)|!dads.iter().any(|x|required_by.contains_key(x))).map(|(k,_v)|k.to_owned()).collect::<Vec<_>>();
            for k in to_pop {
                required_by.remove(&k);
                toposorted.push(k);
            }
        }
        dbg!(&toposorted);
        assert!(toposorted.pop().unwrap()=="ORE");

        let mut upper = 1000000000000i64;
        let mut lower = 0i64;
        while upper>lower+1 {
            let fnum= (upper+lower)/2;

            let mut need: BTreeMap<String, i64> = BTreeMap::new();
            need.insert("FUEL".to_owned(), fnum);
            for key in &toposorted {
                if need.contains_key(key) {
                    let (fqty, ingrs) = &formulas[key];
                    let qty = need[key];
                    for (iname, iq) in ingrs {
                        *need.entry(iname.to_owned()).or_insert(0i64) += iq * ((qty+fqty-1)/fqty);
                    }
                }
            }
            if need["ORE"] >1000000000000 {
                upper = fnum;
            } else {
                lower = fnum;
            }
        }
        res = lower;
    }


    println!("{}", res);
    Ok(())
}
