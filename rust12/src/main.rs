use std::env;
use std::fs;
use std::error::Error;
use num::integer::lcm;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Moon {
    pos: Vec<i64>,
    vel: Vec<i64>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
struct MoonC {
    pos: i64,
    vel: i64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let task_part = &env::args().collect::<Vec<String>>()[1];
    let input = fs::read_to_string("input.txt")?;

    let mut res = 0i64;

    if task_part=="1" {
        let mut moons = vec![];
        for l in input.lines() {
            let xyz = l.trim_end_matches('>').split(", ").map(|x|x.split("=").skip(1).next().unwrap().parse::<i64>().unwrap()).collect();
            moons.push(Moon{pos:xyz, vel:vec![0,0,0]});
        }
        for st in 0..1000 {
            dbg!(st, &moons);
            for i1 in 0..moons.len() {
                for i2 in i1..moons.len() {
                    if i1!=i2 {
                        let (head, tail) = moons.split_at_mut(i1 + 1);
                        let m1 = &mut head[i1];
                        let m2 = &mut tail[i2-i1-1];
                        for i in 0..3 {
                            if m1.pos[i] > m2.pos[i] {
                                m1.vel[i]-=1;
                                m2.vel[i]+=1;
                            }
                            else if m1.pos[i] < m2.pos[i] {
                                m1.vel[i]+=1;
                                m2.vel[i]-=1;
                            }
                        }
                    }
                }
            }
            for m in moons.iter_mut() {
                for i in 0..3 {
                    m.pos[i] += m.vel[i];
                }
            }
        }
        dbg!(&moons);
        res = moons.iter().map::<i64, _>(|m|(m.pos.iter().map(|x|x.abs()).sum::<i64>() * m.vel.iter().map(|x|x.abs()).sum::<i64>())).sum();
    }

    if task_part=="2" {
        let mut moons_all = vec![];
        for l in input.lines() {
            let xyz = l.trim_end_matches('>').split(", ").map(|x|x.split("=").skip(1).next().unwrap().parse::<i64>().unwrap()).collect();
            moons_all.push(Moon{pos:xyz, vel:vec![0,0,0]});
        }
        res = 1;
        for i in 0..3 {
            let moons0 = moons_all.iter().map(|m|MoonC{pos:m.pos[i], vel:m.vel[i]}).collect::<Vec<_>>();
            let mut moons = moons_all.iter().map(|m|MoonC{pos:m.pos[i], vel:m.vel[i]}).collect::<Vec<_>>();
            let mut loopc = 0;
            loop {
                loopc += 1;            
                for i1 in 0..moons.len() {
                    for i2 in i1..moons.len() {
                        if i1!=i2 {
                            let (head, tail) = moons.split_at_mut(i1 + 1);
                            let m1 = &mut head[i1];
                            let m2 = &mut tail[i2-i1-1];
                            if m1.pos > m2.pos {
                                m1.vel-=1;
                                m2.vel+=1;
                            }
                            else if m1.pos < m2.pos {
                                m1.vel+=1;
                                m2.vel-=1;
                            }
                        }
                    }
                }
                for m in moons.iter_mut() {
                    m.pos += m.vel;
                }
                if moons == moons0 {
                    res = lcm(res, loopc);
                    break;
                }
            }
        }
    }


    println!("{}", res);
    Ok(())
}
