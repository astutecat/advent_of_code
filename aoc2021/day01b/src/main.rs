use std::io::{BufReader, BufRead};
use std::{error::Error, i32};

use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let mut window_a = Vec::new();
    let mut window_b = Vec::new();
    let mut num_increased = 0;
    let mut num_decreased = 0;

    let file = File::open("input.txt")?;

    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(l) = line {
            let parsed = l.parse::<i32>()?;
            if window_a.len() < 3 {
                window_a.append(parsed)
            } else if window_b.len() < 3 {
                window_b.append(parsed)
            } else {
                let a_sum = window_a.iter().sum();
                let b_sum = window_b.iter().sum();
                if b_sum > a_sum {
                    num_increased += 1;
                    window_a = window_b;
                    window_b = Vec::new();
                }
            }
        }

    }
    Ok(())
}

