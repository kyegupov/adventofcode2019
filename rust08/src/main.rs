use std::env;
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let task_part = &env::args().collect::<Vec<String>>()[1];
    let input = fs::read_to_string("input.txt")?;
    let chars: Vec<char> = input.chars().collect();


    let mut res = 0;
    if task_part=="1" {
        let size = 25*6;
        let layers = (0..chars.len()/size).map(|x|&chars[x*size..(x+1)*size]);
        let minl = layers.min_by_key(|l|l.iter().filter(|c|**c=='0').count()).unwrap();
        res = minl.iter().filter(|c|**c=='1').count() * minl.iter().filter(|c|**c=='2').count();
    }

    if task_part=="2" {
        let size = 25*6;
        let layers: Vec<_> = (0..chars.len()/size).map(|x|&chars[x*size..(x+1)*size]).collect();
        let mut out = vec![];
        for y in 0..6 {
            let mut row = String::new();
            for x in 0..25 {
                let i = y*25+x;
                let c = layers.iter().map(|l|l[i]).find(|c|*c!='2').unwrap();
                row.push(if c == '1' {'*'} else {' '});
            }
            out.push(row);
        }
        println!("{}", out.join("\n"));
    }


    println!("{}", res);
    Ok(())
}
