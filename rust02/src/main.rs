use std::env;
use std::fs;
use std::error::Error;
use std::collections::BTreeSet;

fn main() -> Result<(), Box<dyn Error>> {
    let task_part = &env::args().collect::<Vec<String>>()[1];
    let input = fs::read_to_string("input.txt")?;
    let data: Vec<usize> = input.split(',').map(|x|x.parse().unwrap()).collect();

    let mut res = 0;
    if task_part=="1" {
        let mut a = data.clone();
        a[1] = 12;
        a[2] = 2;
        let mut i = 0;
        loop {
            match a[i] {
                1 => {
                    if let [x,y,z] = a[i+1..i+4] {
                        a[z] = a[x]+a[y];
                    }
                },
                2 => {

                    if let [x,y,z] = a[i+1..i+4] {
                        a[z] = a[x]*a[y];
                    }
                },
                99 => {break;},
                _ => panic!()
            }
            i += 4;
        }
        res = a[0];
    }

    if task_part=="2" {
        for noun in 0..99 {
            for verb in 0..99 {
                let mut a = data.clone();
                a[1] = noun;
                a[2] = verb;

                let mut i = 0;
                loop {
                    match a[i] {
                        1 => {
                            if let [x,y,z] = a[i+1..i+4] {
                                a[z] = a[x]+a[y];
                            }
                        },
                        2 => {

                            if let [x,y,z] = a[i+1..i+4] {
                                a[z] = a[x]*a[y];
                            }
                        },
                        99 => {break;},
                        _ => panic!()
                    }
                    i += 4;
                }
                if a[0] == 19690720 {
                    res = noun*100+verb;
                    break;
                }
            }
        }
    }


    println!("{}", res);
    Ok(())
}
