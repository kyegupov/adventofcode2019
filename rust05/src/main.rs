use std::env;
use std::fs;
use std::error::Error;
use std::collections::BTreeSet;

fn main() -> Result<(), Box<dyn Error>> {
    let task_part = &env::args().collect::<Vec<String>>()[1];
    let input = fs::read_to_string("input.txt")?;
    let data: Vec<i32> = input.split(',').map(|x|x.parse().unwrap()).collect();

    let mut res = 0;
    if task_part=="1" {
        let mut a = data.clone();
        let mut i = 0;
        loop {
            match a[i]%100 {
                1 => {
                    if let [mut x, mut y,z] = a[i+1..i+4] {
                        if (a[i]/100)%10 == 0 {
                            x = a[x as usize];
                        }
                        if (a[i]/1000)%10 == 0 {
                            y = a[y as usize];
                        }
                        a[z as usize] = x+y
                    }
                    i += 4;
                },
                2 => {

                    if let [mut x, mut y,z] = a[i+1..i+4] {
                        if (a[i]/100)%10 == 0 {
                            x = a[x as usize];
                        }
                        if (a[i]/1000)%10 == 0 {
                            y = a[y as usize];
                        }
                        a[z as usize] = x*y
                    }
                    i += 4;
                },
                3 => {
                    let z = a[i+1] as usize;
                    a[z] = 1;
                    i += 2;

                },
                4 => {

                    println!("{}", a[a[i+1] as usize]);
                    i += 2;

                },

                99 => {break;},
                _ => panic!(a[i].to_string())
            }
            // println!("{:?}", a);
        }
        res = a[0];
    }

    if task_part=="2" {
        let mut a = data.clone();
        let mut i = 0;
        loop {
            match a[i]%100 {
                1 => {
                    if let [mut x, mut y,z] = a[i+1..i+4] {
                        if (a[i]/100)%10 == 0 {
                            x = a[x as usize];
                        }
                        if (a[i]/1000)%10 == 0 {
                            y = a[y as usize];
                        }
                        a[z as usize] = x+y
                    }
                    i += 4;
                },
                2 => {

                    if let [mut x, mut y,z] = a[i+1..i+4] {
                        if (a[i]/100)%10 == 0 {
                            x = a[x as usize];
                        }
                        if (a[i]/1000)%10 == 0 {
                            y = a[y as usize];
                        }
                        a[z as usize] = x*y
                    }
                    i += 4;
                },
                3 => {
                    let z = a[i+1] as usize;
                    a[z] = 5;
                    i += 2;

                },
                4 => {

                    println!("{}", a[a[i+1] as usize]);
                    i += 2;

                },
                5 => {
                    if let [mut x, mut y] = a[i+1..i+3] {
                        if (a[i]/100)%10 == 0 {
                            x = a[x as usize];
                        }
                        if (a[i]/1000)%10 == 0 {
                            y = a[y as usize];
                        }
                        if x != 0 {
                            i = y as usize;
                            continue;
                        }
                    }
                    i += 3;
                },
                6 => {
                    if let [mut x, mut y] = a[i+1..i+3] {
                        if (a[i]/100)%10 == 0 {
                            x = a[x as usize];
                        }
                        if (a[i]/1000)%10 == 0 {
                            y = a[y as usize];
                        }
                        if x == 0 {
                            i = y as usize;
                            continue;
                        }
                    }
                    i += 3;
                },
                7 => {
                    if let [mut x, mut y,z] = a[i+1..i+4] {
                        if (a[i]/100)%10 == 0 {
                            x = a[x as usize];
                        }
                        if (a[i]/1000)%10 == 0 {
                            y = a[y as usize];
                        }
                        a[z as usize] = if x<y {1} else {0};
                    }
                    i += 4;
                },
                8 => {
                    if let [mut x, mut y,z] = a[i+1..i+4] {
                        if (a[i]/100)%10 == 0 {
                            x = a[x as usize];
                        }
                        if (a[i]/1000)%10 == 0 {
                            y = a[y as usize];
                        }
                        a[z as usize] = if x==y {1} else {0};
                    }
                    i += 4;
                },
                99 => {break;},
                _ => panic!(a[i].to_string())
            }
            // println!("{:?} {}", a, i);
        }
        res = a[0];
    }


    // println!("{}", res);
    Ok(())
}
