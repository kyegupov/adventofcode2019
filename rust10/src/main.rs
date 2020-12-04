use std::env;
use std::fs;
use std::error::Error;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use num::integer::gcd;
use ordered_float::OrderedFloat;

fn main() -> Result<(), Box<dyn Error>> {
    let task_part = &env::args().collect::<Vec<String>>()[1];
    let input = fs::read_to_string("input.txt")?;
    let lines: Vec<_> = input.lines().map(|x|x.chars().collect::<Vec<_>>()).collect();

    let mut res = 0;
    if task_part=="1" {
        let mut blocked = BTreeMap::new();
        for y in 0..lines.len() {
            for x in 0..lines[0].len() {
                if lines[y][x] == '#' {
                    let mut set = BTreeSet::new();
                    for y2 in 0..lines.len() {
                        for x2 in 0..lines[0].len() {
                            if (y2 != y || x2!=x) && lines[y2][x2] == '#' {
                                let mut dx = x2 as i32 - x as i32;
                                let mut dy = y2 as i32 - y as i32;
                                if x==8 && y==0 {
                                    // dbg!(dx,dy);
                                }
                                let g =gcd(dx,dy);
                                dx /= g;
                                dy /= g;
                                if x==8 && y==0 {
                                    // dbg!(dx,dy);
                                }
                                for k in 1..100000 {
                                    let x3 = x2 as i32+(k*dx);
                                    let y3 = y2 as i32+(k*dy);
                                    if x==4 && y==7 && dx==-1 &&dy==-1 {
                                        // dbg!(x3,y3);
                                    }
                                    if y3 < lines.len() as i32 && y3 >= 0 && x3 < lines[0].len() as i32 && x3 >= 0 {
                                        set.insert((x3 as usize, y3 as usize));
                                    } else {
                                        break;
                                    }
                                }
                                if x==4 && y==7 && dx==-1 &&dy==-1 {
                                    // dbg!(&set);
                                }
                            }
                        }
                    }
                    blocked.insert((x as usize, y as usize), set);
                }
            }
        }
        // dbg!(&blocked);
        for ((x0, y0), set) in blocked.iter() {
            let count = blocked.keys().filter(|(x2,y2)|!set.contains(&(*x2, *y2))).count()-1;

            if count > res {
                // dbg!(x0,y0);
                res = count;
                // println!("=======");
                // for y in 0..lines.len() {
                //     for x in 0..lines[0].len() {
                //         if x==*x0 && y==*y0 {
                //             print!("0")
                //         }
                //         else if lines[y][x] == '#' {
                //             if set.contains(&(x,y)) {
                //                 print!("X")
                //             } else {
                //                 print!("#")
                //             }
                //         } else {
                //             print!(".");
                //         }
                //     }
                //     println!();
                // }
            }
        }
    }

    if task_part=="2" {
        let x = 11;
        let y = 11;
        let mut blocked_by = BTreeMap::new();
        let mut asteroids = BTreeSet::new();
        for y2 in 0..lines.len() {
            for x2 in 0..lines[0].len() {
                if (y2 != y || x2!=x) && lines[y2][x2] == '#' {
                    asteroids.insert((x2 as i32-x as i32,y2 as i32-y as i32));
                    let mut dx = x2 as i32 - x as i32;
                    let mut dy = y2 as i32 - y as i32;
                    let g =gcd(dx,dy);
                    dx /= g;
                    dy /= g;
                    for k in 1..100000 {
                        let x3 = x2 as i32+(k*dx);
                        let y3 = y2 as i32+(k*dy);
                        if y3 < lines.len() as i32 && y3 >= 0 && x3 < lines[0].len() as i32 && x3 >= 0 {
                            blocked_by.entry((x3 as i32-x as i32, y3 as i32-y as i32)).or_insert(vec![]).push((x2 as i32-x as i32,y2 as i32-x as i32));
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        let mut nonblocked = asteroids.iter().filter(|a|blocked_by.get(a).unwrap_or(&vec![]).is_empty()).collect::<Vec<_>>();
        nonblocked.sort_by_key(|(dx, dy)|OrderedFloat((-(*dx as f64)).atan2((*dy as f64))));
        let (rx, ry) = &nonblocked[199];
        res = ((*rx+x as i32)*100+(*ry+y as i32)) as usize;
    }


    println!("{}", res);
    Ok(())
}
