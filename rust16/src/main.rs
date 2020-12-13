use std::env;
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let task_part = &env::args().collect::<Vec<String>>()[1];
    let input = fs::read_to_string("input.txt")?.trim().to_owned();

    let mut res = 0;
    let seq = [0,1,0,-1];
    let mut s = input.clone();
    if task_part=="1" {
        for k in 0..100 {
            let mut s2 = String::new();
            for i in 0..input.len() {
                let mut sum = 0i64;
                for (j,c) in s.chars().enumerate() {
                    let lineno = i + 1;
                    let modpos = (j+1)%(lineno*4)/lineno;
                    let mult = seq[modpos];
                    // dbg!(j, modpos, mult);
                    sum += mult * c.to_string().parse::<i64>().unwrap();
                }
                s2.push_str(&(sum.abs() % 10).to_string());
            }
            s = s2;
        }
        dbg!(&s[0..8]);
    }

    if task_part=="2" {
        let mut orig: Vec<i64> = input.chars().map(|c|c.to_string().parse::<i64>().unwrap()).collect();
        let mut s = vec![];
        for k in 0..10000 {
            s.extend_from_slice(&orig);
        }
        println!("start");
        for k in 0..100 {
            let mut s2 = s.clone();
            let mut acc = 0;
            for i in ((s.len()+1)/2..s.len()).rev() {
                acc += s[i];
                s2[i] = acc % 10;
            }
            s = s2;
            dbg!(k);
        }
        let offs = input[0..7].parse::<usize>().unwrap();
        let out = &s[offs..offs+8];
        dbg!(out);
    }


    println!("{}", res);
    Ok(())
}
