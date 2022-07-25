use std::env;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_u32(line: &String) -> u32 {
    line.parse::<u32>().unwrap()
}

pub fn growing_depths() -> std::io::Result<i32> {

    let mut path = std::path::PathBuf::new();

    if let Some(filepath) = env::args().last() {
        path.push(filepath);
    }

    let file = File::open(path)?;
    let buff = BufReader::new(&file);
    
    let mut biter = buff.lines().into_iter();
    let mut first = parse_u32(&biter.next().unwrap()?);
    let mut count = 0;

    for line in biter {

        let second = parse_u32(&line.unwrap());
        if second > first { count += 1 };
        first = second;

    }

    Ok(count) // 1374 ✅
}
