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

    let mut board = BTreeMap::new();
    board.insert((0,0), 1);

    let mut vm = Vm{a: data.clone(), i:0, halt: false, relbase: 0};

    let mut sides0 = BTreeMap::new(); // dx, dy -> there, dback
    sides0.insert((0, -1), (1, 2));
    sides0.insert((0, 1), (2, 1));
    sides0.insert((-1, 0), (3, 4));
    sides0.insert((1, 0), (4, 3));
    let sides = sides0;


    fn dfs(x: i64, y:i64, board: &mut BTreeMap<(i64, i64),i64>, sides: &BTreeMap<(i64, i64), (i64, i64)>, vm: &mut Vm) {
        for ((dx,dy), (fwd, bak)) in sides {
            if !board.contains_key(&(x+dx, y+dy)) {
                let out = run_program(vm, vec![*fwd]);
                board.insert((x+dx, y+dy), out[0]);
                if out[0] > 0 {
                    dfs(x+dx,y+dy, board, sides, vm);
                    let out = run_program(vm, vec![*bak]);
                }
            }
        }
    }
    dfs(0,0, &mut board, &sides, &mut vm);
    let ymin = board.keys().map(|k|k.1).min().unwrap();
    let ymax = board.keys().map(|k|k.1).max().unwrap();
    let xmin = board.keys().map(|k|k.0).min().unwrap();
    let xmax = board.keys().map(|k|k.0).max().unwrap();
    for y in ymin..=ymax {
        for x in xmin..=xmax {
            let c = match board.get(&(x,y)).unwrap_or(&-1) {
                0 => '#',
                1 => '.',
                2 => '!',
                -1 => ' ',
                _ => panic!(),
            };
            print!("{}", c);
        }
        println!("");
    }


    let mut board2 = BTreeMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((0,0));
    board2.insert((0,0), 0);
    while queue.len() > 0 {
        let (x, y) = queue.pop_front().unwrap();
        if *board.get(&(x, y)).unwrap() == 2 {
            dbg!(x,y);
            res = board2[&(x, y)];
            break;
        }
        for ((dx,dy), (fwd, bak)) in &sides {
            if !board2.contains_key(&(x+dx, y+dy)) && *board.get(&(x+dx, y+dy)).unwrap_or(&-1) >0 {
                board2.insert((x+dx, y+dy), board2[&(x, y)]+1);
                queue.push_back((x+dx, y+dy));
            }
        }
    }
    dbg!(res);

    if task_part=="2" {

        board2 = BTreeMap::new();
        queue = VecDeque::new();
        let (x,y) = board.iter().find(|(k,v)|**v==2).unwrap().0;
        board2.insert((*x,*y), 0);
        queue.push_back((*x,*y));
        while queue.len() > 0 {
            let (x, y) = queue.pop_front().unwrap();
            for ((dx,dy), (fwd, bak)) in &sides {
                if !board2.contains_key(&(x+dx, y+dy)) && *board.get(&(x+dx, y+dy)).unwrap_or(&-1) >0 {
                    board2.insert((x+dx, y+dy), board2[&(x, y)]+1);
                    res = board2[&(x, y)]+1;
                    queue.push_back((x+dx, y+dy));
                }
            }
        }

    }




    println!("RES {}", res);
    Ok(())
}
