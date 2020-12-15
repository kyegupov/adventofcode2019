use std::env;
use std::fs;
use std::error::Error;
use std::collections::{BTreeMap, VecDeque};
#[derive(Debug)]
struct Vm {
    a: Vec<i64>,
    i: usize,
    relbase: usize,
    halt: bool,
}

fn pull_args(vm: &mut Vm, modifs: &mut i64, n: usize) -> Vec<i64> {
    let mut res = vec![];
    for _ in 0..n {
        let modif = *modifs%10;
        *modifs /= 10;
        let mut x = vm.a[vm.i];
        if modif == 0 {
            let ii = x as usize;
            // dbg!(vm.i, ii);
            while ii > vm.a.len()-1 {
                vm.a.push(0);
            }
            x = vm.a[ii];
        } else if modif == 2 {
            let ii = (vm.relbase as i64 + x) as usize;
            // dbg!(ii);
            while ii > vm.a.len()-1 {
                vm.a.push(0);
            }
            x = vm.a[ii];
        }
        res.push(x);
        vm.i += 1;
    }
    res
}

fn store_into_target(vm: &mut Vm,modifs: &mut i64,  value: i64) {
    let x = vm.a[vm.i];
    let modif = *modifs%10;
    if modif == 0 {
        let ii = x as usize;
        while ii > vm.a.len()-1 {
            vm.a.push(0);
        }
        vm.a[ii] = value;
    } else if modif == 2 {
        let ii = (vm.relbase as i64 + x) as usize;
        while ii > vm.a.len()-1 {
            vm.a.push(0);
        }
        vm.a[ii] = value;

    }
    vm.i += 1;
}

fn run_program(vm: &mut Vm, inputs: Vec<i64>) -> Vec<i64> {
    let mut inp_pos = 0;
    let mut outputs = vec![];
    loop {
        let op = vm.a[vm.i];
        // dbg!(vm.i, op);
        vm.i += 1;
        let mut modifs = op/100;
        match op%100 {
            1 => {
                if let [x,y] = pull_args(vm, &mut modifs, 2)[..] {
                    store_into_target(vm, &mut modifs, x+y);
                } else { panic!(); }
            },
            2 => {
                if let [x,y] = pull_args(vm, &mut modifs, 2)[..] {
                    store_into_target(vm, &mut modifs,  x*y);
                } else { panic!(); }
            },
            3 => {
                if inp_pos >= inputs.len() {
                    vm.i -= 1;
                    return outputs;
                }
                store_into_target(vm, &mut modifs, inputs[inp_pos]);
                inp_pos+=1;
            },
            4 => {
                if let [x] = pull_args(vm, &mut modifs, 1)[..] {
                    // dbg!(x, &outputs);
                    outputs.push(x);
                } else { panic!(); }
            },
            5 => {
                if let [x,y] = pull_args(vm, &mut modifs, 2)[..] {
                    if x != 0 {
                        vm.i = y as usize;
                        continue;
                    }
                } else { panic!(); }
            },
            6 => {
                if let [x,y] = pull_args(vm, &mut modifs, 2)[..] {
                    if x == 0 {
                        vm.i = y as usize;
                        continue;
                    }
                } else { panic!(); }
            },
            7 => {
                if let [x,y] = pull_args(vm, &mut modifs,2)[..] {
                    store_into_target(vm, &mut modifs, if x<y {1} else {0});
                } else { panic!(); }
            },
            8 => {
                if let [x,y] = pull_args(vm, &mut modifs, 2)[..] {
                    store_into_target(vm, &mut modifs, if x==y {1} else {0});
                } else { panic!(); }
            },
            9 => {
                if let [x] = pull_args(vm, &mut modifs, 1)[..] {
                    let mut r = vm.relbase as i64;
                    r += x;
                    vm.relbase = r as usize;
                } else { panic!(); }
            },
            99 => {vm.halt = true; return outputs;},
            _ => panic!(op.to_string())
        }
        // dbg!(&vm);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let task_part = &env::args().collect::<Vec<String>>()[1];
    let input = fs::read_to_string("input.txt")?;
    let data: Vec<i64> = input.split(',').map(|x|x.parse().unwrap()).collect();

    let mut res = 0i64;
    let cross = vec![(0,0),(0,-1),(1,0),(0,1),(-1,0)];
    let sides = vec![(0,-1),(1,0),(0,1),(-1,0)];

    if task_part == "1" {
    
        let mut vm = Vm{a: data.clone(), i:0, halt: false, relbase: 0};

        let mut y = 0;
        let mut x = 0;

        let out = run_program(&mut vm, vec![]);

        let mut board = BTreeMap::new();

        for c in out {
            if c as u8 as char == '\n' {
                y += 1;
                x = 0;
            } else {
                board.insert((x,y), c as u8 as char);
                x += 1;
            }
            print!("{}",c as u8 as char);
        }
        let mut board2 = board.clone();
        for ((x,y),c) in &board {
            if cross.iter().all(|(dx,dy)|*board.get(&(x+dx, y+dy)).unwrap_or(&' ')=='#') {
                dbg!(x,y, x*y);
                res += x*y;
                board2.insert((*x,*y), 'O');
            }
        }

        let ymin = board2.keys().map(|k|k.1).min().unwrap();
        let ymax = board2.keys().map(|k|k.1).max().unwrap();
        let xmin = board2.keys().map(|k|k.0).min().unwrap();
        let xmax = board2.keys().map(|k|k.0).max().unwrap();
        for y in ymin..=ymax {
            for x in xmin..=xmax {
                let c = board2.get(&(x,y)).unwrap_or(&' ');
                print!("{}", c);
            }
            println!("");
        }
            
    
    }


    if task_part == "2" {
    
        let mut vm = Vm{a: data.clone(), i:0, halt: false, relbase: 0};
        vm.a[0] = 2;

        let mut y = 0;
        let mut x = 0;

        let out = run_program(&mut vm, vec![]);

        let mut board = BTreeMap::new();

        for c in out {
            if c as u8 as char == '\n' {
                y += 1;
                x = 0;
            } else {
                board.insert((x,y), c as u8 as char);
                x += 1;
            }
            print!("{}",c as u8 as char);
        }
        let mut board2 = board.clone();
        
        let bot_xy = *board.iter().find(|((x,y),c)|**c=='^').unwrap().0;
        x=bot_xy.0;
        y=bot_xy.1;

        let (mut dx,mut dy) = (0,-1);
        
        dbg!(x,y);

        let mut code = vec![];
        let ymin = board.keys().map(|k|k.1).min().unwrap();
        let ymax = board.keys().map(|k|k.1).max().unwrap();
        let xmin = board.keys().map(|k|k.0).min().unwrap();
        let xmax = board.keys().map(|k|k.0).max().unwrap();

        loop {
            let mut dir = "B";
            
            if *board.get(&(x-dy, y+dx)).unwrap_or(&' ')=='#' {
                dir = "R";
                let t = dx;
                dx = -dy;
                dy = t;
            } else if *board.get(&(x+dy, y-dx)).unwrap_or(&' ')=='#'  {
                dir = "L";
                let t = dx;
                dx = dy;
                dy = -t;
            } else {
                break;
            }
            let mut steps = 0;
            while *board.get(&(x+dx, y+dy)).unwrap_or(&' ')=='#' {
                steps += 1;
                x += dx;
                y += dy;
                board2.insert((x,y),'x');
            }
            dbg!(x,y,dx,dy,steps);
            code.push((dir, steps));
            for y in ymin..=ymax {
                for x in xmin..=xmax {
                    let c = board2.get(&(x,y)).unwrap_or(&' ');
                    print!("{}", c);
                }
                println!("");
            }
        }
        println!("{}", code.iter().map(|(op,num)|op.to_string()+","+&num.to_string()).collect::<Vec<_>>().join(","));

        vm = Vm{a: data.clone(), i:0, halt: false, relbase: 0};
        vm.a[0] = 2;

        let program: Vec<i64> = "A,B,A,B,A,C,B,C,A,C
L,6,R,12,L,6
R,12,L,10,L,4,L,6
L,10,L,10,L,4,L,6
n
".chars().map(|x|x as u8 as i64).collect();

        let out = run_program(&mut vm, program);
        for c in &out {
            if *c as u8 as char == '\n' {
                y += 1;
                x = 0;
            } else {
                board.insert((x,y), *c as u8 as char);
                x += 1;
            }
            print!("{}",*c as u8 as char);
        }
        println!("{}", &out.last().unwrap());

    }



    println!("RES {}", res);
    Ok(())
}
