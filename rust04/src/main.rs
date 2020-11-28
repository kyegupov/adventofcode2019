use std::env;
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let task_part = &env::args().collect::<Vec<String>>()[1];
    // let input = fs::read_to_string("input")?;

    let mut res = 0;
    if task_part=="1" {
        for i in 278384..=824795 {
            let digits: Vec<char> = i.to_string().chars().collect();
            let twoadj = (0..digits.len()-1).any(|i|digits[i]==digits[i+1]);
            let increases = (0..digits.len()-1).all(|i|digits[i]<=digits[i+1]);
            if twoadj && increases {
                // println!("{}", i);
                res+=1;
            }
        }
    }

    if task_part=="2" {
        for i in 278384..=824795 {
            let digits: Vec<char> = i.to_string().chars().collect();
            let mut p = 'x';
            let mut streak = 1;
            let mut has_streak_2 = false;
            for d in digits.iter() {
                if *d==p {
                    streak +=1;
                } else {
                    p=*d;
                    if streak==2 {
                        has_streak_2 = true;
                        break;
                    }
                    streak = 1;
                }
            }
            if streak==2 {
                has_streak_2 = true;
            }


            let increases = (0..digits.len()-1).all(|i|digits[i]<=digits[i+1]);
            if has_streak_2 && increases {
                // println!("{}", i);
                res+=1;
            }
        }
    }


    println!("{}", res);
    Ok(())
}
