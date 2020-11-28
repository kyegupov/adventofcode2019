use std::env;
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let task_part = &env::args().collect::<Vec<String>>()[1];
    let input = fs::read_to_string("input")?;

    let mut res = 0;
    if task_part=="1" {
        for i in input.lines().map(|x|x.parse::<i32>().unwrap()) {
            res += (i/3)-2;
        }
    }

    if task_part=="2" {
        for mut i in input.lines().map(|x|x.parse::<i32>().unwrap()) {
            loop {
                i = (i/3)-2;
                if i <= 0 {
                    break;
                }
                res += i;
            }
        }
    }


    println!("{}", res);
    Ok(())
}
