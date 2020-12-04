use std::env;
use std::fs;
use std::error::Error;
use std::collections::BTreeSet;
use itertools::Itertools;

struct Amp {
    a: Vec<i32>,
    i: usize,
    halt: bool,
}

fn run_program(amp: &mut Amp, inputs: Vec<i32>) -> Vec<i32> {
    let mut inp_pos = 0;
    let mut outputs = vec![];
    loop {
        let op = amp.a[amp.i];
        match op%100 {
            1 => {
                if let [mut x, mut y,z] = amp.a[amp.i+1..amp.i+4] {
                    if (amp.a[amp.i]/100)%10 == 0 {
                        x = amp.a[x as usize];
                    }
                    if (amp.a[amp.i]/1000)%10 == 0 {
                        y = amp.a[y as usize];
                    }
                    amp.a[z as usize] = x+y
                }
                amp.i += 4;
            },
            2 => {

                if let [mut x, mut y,z] = amp.a[amp.i+1..amp.i+4] {
                    if (amp.a[amp.i]/100)%10 == 0 {
                        x = amp.a[x as usize];
                    }
                    if (amp.a[amp.i]/1000)%10 == 0 {
                        y = amp.a[y as usize];
                    }
                    amp.a[z as usize] = x*y
                }
                amp.i += 4;
            },
            3 => {
                let z = amp.a[amp.i+1] as usize;
                if inp_pos >= inputs.len() {
                    return outputs;
                }
                amp.a[z] = inputs[inp_pos];
                inp_pos+=1;
                amp.i += 2;

            },
            4 => {
                let mut x= amp.a[amp.i+1];
                if (amp.a[amp.i]/100)%10 == 0 {
                    x = amp.a[x as usize];
                }
                outputs.push(x);
                amp.i += 2;
            },
            5 => {
                if let [mut x, mut y] = amp.a[amp.i+1..amp.i+3] {
                    if (amp.a[amp.i]/100)%10 == 0 {
                        x = amp.a[x as usize];
                    }
                    if (amp.a[amp.i]/1000)%10 == 0 {
                        y = amp.a[y as usize];
                    }
                    if x != 0 {
                        amp.i = y as usize;
                        continue;
                    }
                }
                amp.i += 3;
            },
            6 => {
                if let [mut x, mut y] = amp.a[amp.i+1..amp.i+3] {
                    if (amp.a[amp.i]/100)%10 == 0 {
                        x = amp.a[x as usize];
                    }
                    if (amp.a[amp.i]/1000)%10 == 0 {
                        y = amp.a[y as usize];
                    }
                    if x == 0 {
                        amp.i = y as usize;
                        continue;
                    }
                }
                amp.i += 3;
            },
            7 => {
                if let [mut x, mut y,z] = amp.a[amp.i+1..amp.i+4] {
                    if (amp.a[amp.i]/100)%10 == 0 {
                        x = amp.a[x as usize];
                    }
                    if (amp.a[amp.i]/1000)%10 == 0 {
                        y = amp.a[y as usize];
                    }
                    amp.a[z as usize] = if x<y {1} else {0};
                }
                amp.i += 4;
            },
            8 => {
                if let [mut x, mut y,z] = amp.a[amp.i+1..amp.i+4] {
                    if (amp.a[amp.i]/100)%10 == 0 {
                        x = amp.a[x as usize];
                    }
                    if (amp.a[amp.i]/1000)%10 == 0 {
                        y = amp.a[y as usize];
                    }
                    amp.a[z as usize] = if x==y {1} else {0};
                }
                amp.i += 4;
            },
            99 => {amp.halt = true; return outputs;},
            _ => panic!(amp.a[amp.i].to_string())
        }
        // println!("{:?} {}", a, i);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let task_part = &env::args().collect::<Vec<String>>()[1];
    let input = fs::read_to_string("input.txt")?;
    let data: Vec<i32> = input.split(',').map(|x|x.parse().unwrap()).collect();

    let mut res = 0;

    if task_part=="1" {

        for phases in (0..5).permutations(5) {
            // dbg!(phases);
            let mut signal = 0;
            for p in phases.iter() {
                let mut amp = Amp{a: data.clone(), i:0, halt: false};
                signal = run_program(&mut amp, vec![*p, signal])[0];
            }
            if res < signal {
                res = signal;
            }
        }
    }

    if task_part=="2" {

        for phases in (5..10).permutations(5) {
            // dbg!(phases);
            let mut signals = vec![0];
            let mut amps: Vec<_> = (0..5).map(|_|Amp{a: data.clone(), i:0, halt: false}).collect();
            let mut first = true;
            while !amps[4].halt {
                for (p, mut amp) in phases.iter().zip(amps.iter_mut()) {
                    let mut input: Vec<i32> = vec![];
                    if first {
                        input.push(*p);
                    }
                    input.extend(&signals);
                    signals = run_program(&mut amp, input);
                }
                first = false;
            }
            // dbg!(phases, &signals);
            assert!(signals.len()==1);
            if res < signals[0] {
                res = signals[0];
            }
        }
    }


    println!("{}", res);
    Ok(())
}
