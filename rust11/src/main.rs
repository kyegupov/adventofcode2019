use std::env;
use std::fs;
use std::error::Error;
use std::collections::BTreeMap;

#[derive(Debug)]
struct Amp {
    a: Vec<i64>,
    i: usize,
    relbase: usize,
    halt: bool,
}

fn pull_args(amp: &mut Amp, modifs: &mut i64, n: usize) -> Vec<i64> {
    let mut res = vec![];
    for _ in 0..n {
        let modif = *modifs%10;
        *modifs /= 10;
        let mut x = amp.a[amp.i];
        if modif == 0 {
            let ii = x as usize;
            // dbg!(amp.i, ii);
            while ii > amp.a.len()-1 {
                amp.a.push(0);
            }
            x = amp.a[ii];
        } else if modif == 2 {
            let ii = (amp.relbase as i64 + x) as usize;
            // dbg!(ii);
            while ii > amp.a.len()-1 {
                amp.a.push(0);
            }
            x = amp.a[ii];
        }
        res.push(x);
        amp.i += 1;
    }
    res
}

fn store_into_target(amp: &mut Amp,modifs: &mut i64,  value: i64) {
    let x = amp.a[amp.i];
    let modif = *modifs%10;
    if modif == 0 {
        let ii = x as usize;
        while ii > amp.a.len()-1 {
            amp.a.push(0);
        }
        amp.a[ii] = value;
    } else if modif == 2 {
        let ii = (amp.relbase as i64 + x) as usize;
        while ii > amp.a.len()-1 {
            amp.a.push(0);
        }
        amp.a[ii] = value;

    }
    amp.i += 1;
}

fn run_program(amp: &mut Amp, inputs: Vec<i64>) -> Vec<i64> {
    let mut inp_pos = 0;
    let mut outputs = vec![];
    loop {
        let op = amp.a[amp.i];
        // dbg!(amp.i, op);
        amp.i += 1;
        let mut modifs = op/100;
        match op%100 {
            1 => {
                if let [x,y] = pull_args(amp, &mut modifs, 2)[..] {
                    store_into_target(amp, &mut modifs, x+y);
                } else { panic!(); }
            },
            2 => {
                if let [x,y] = pull_args(amp, &mut modifs, 2)[..] {
                    store_into_target(amp, &mut modifs,  x*y);
                } else { panic!(); }
            },
            3 => {
                if inp_pos >= inputs.len() {
                    amp.i -= 1;
                    return outputs;
                }
                store_into_target(amp, &mut modifs, inputs[inp_pos]);
                inp_pos+=1;
            },
            4 => {
                if let [x] = pull_args(amp, &mut modifs, 1)[..] {
                    // dbg!(x, &outputs);
                    outputs.push(x);
                } else { panic!(); }
            },
            5 => {
                if let [x,y] = pull_args(amp, &mut modifs, 2)[..] {
                    if x != 0 {
                        amp.i = y as usize;
                        continue;
                    }
                } else { panic!(); }
            },
            6 => {
                if let [x,y] = pull_args(amp, &mut modifs, 2)[..] {
                    if x == 0 {
                        amp.i = y as usize;
                        continue;
                    }
                } else { panic!(); }
            },
            7 => {
                if let [x,y] = pull_args(amp, &mut modifs,2)[..] {
                    store_into_target(amp, &mut modifs, if x<y {1} else {0});
                } else { panic!(); }
            },
            8 => {
                if let [x,y] = pull_args(amp, &mut modifs, 2)[..] {
                    store_into_target(amp, &mut modifs, if x==y {1} else {0});
                } else { panic!(); }
            },
            9 => {
                if let [x] = pull_args(amp, &mut modifs, 1)[..] {
                    let mut r = amp.relbase as i64;
                    r += x;
                    amp.relbase = r as usize;
                } else { panic!(); }
            },
            99 => {amp.halt = true; return outputs;},
            _ => panic!(op.to_string())
        }
        // dbg!(&amp);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let task_part = &env::args().collect::<Vec<String>>()[1];
    let input = fs::read_to_string("input.txt")?;
    let data: Vec<i64> = input.split(',').map(|x|x.parse().unwrap()).collect();

    let mut res = 0;

    if task_part=="1" {

        let mut amp = Amp{a: data.clone(), i:0, halt: false, relbase: 0};
        let (mut x, mut y) = (0,0);
        let (mut dx, mut dy) = (0, -1);
        let mut board = BTreeMap::new();

        while !amp.halt {

            let outs = run_program(&mut amp, vec![*board.get(&(x,y)).unwrap_or(&0)]);
            // if outs[0] > 0 {
                board.insert((x,y), outs[0]);
            // }
            if outs[1] == 0 {
                let s = dx;
                dx = dy;
                dy = -s;
            };
            if outs[1] == 1 {
                let s = dx;
                dx = -dy;
                dy = s;
            };

            x += dx;
            y += dy;
            println!("x={},y={}", x, y);
        }
        res = board.len();
        let xmin = board.keys().map(|x|x.0).min().unwrap();
        let xmax = board.keys().map(|x|x.0).max().unwrap();
        let ymin = board.keys().map(|x|x.1).min().unwrap();
        let ymax = board.keys().map(|x|x.1).max().unwrap();
        for y in ymin..=ymax {
            for x in xmin..=xmax {
                print!("{}", if *board.get(&(x,y)).unwrap_or(&0)>0 {"#"} else {"."});
            }
            println!();
        }
    }

    if task_part=="2" {
        let mut amp = Amp{a: data.clone(), i:0, halt: false, relbase: 0};
        let (mut x, mut y) = (0,0);
        let (mut dx, mut dy) = (0, -1);
        let mut board = BTreeMap::new();

        board.insert((0,0), 1);

        while !amp.halt {

            let outs = run_program(&mut amp, vec![*board.get(&(x,y)).unwrap_or(&0)]);
            // if outs[0] > 0 {
                board.insert((x,y), outs[0]);
            // }
            if outs[1] == 0 {
                let s = dx;
                dx = dy;
                dy = -s;
            };
            if outs[1] == 1 {
                let s = dx;
                dx = -dy;
                dy = s;
            };

            x += dx;
            y += dy;
            println!("x={},y={}", x, y);
        }
        res = board.len();
        let xmin = board.keys().map(|x|x.0).min().unwrap();
        let xmax = board.keys().map(|x|x.0).max().unwrap();
        let ymin = board.keys().map(|x|x.1).min().unwrap();
        let ymax = board.keys().map(|x|x.1).max().unwrap();
        for y in ymin..=ymax {
            for x in xmin..=xmax {
                print!("{}", if *board.get(&(x,y)).unwrap_or(&0)>0 {"#"} else {"."});
            }
            println!();
        }


    }


    println!("RES {}", res);
    Ok(())
}
