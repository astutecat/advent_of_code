use std::io::{BufRead, BufReader};
use std::{error::Error, i32};

use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let mut window_a = Vec::new();
    let mut window_b = Vec::new();
    let mut num_increased = 0;
    let mut num_decreased = 0;
    let mut no_change = 0;

    let file = File::open("input.txt")?;

    let reader = BufReader::new(file);

    let lines:Vec<i32> = reader.lines().map(|x| x.unwrap().parse::<i32>().unwrap()).collect();

    println!(
        "decreased: {}, increased: {}, no change: {}",
        num_increased, num_decreased, no_change
    );
    Ok(())
}

