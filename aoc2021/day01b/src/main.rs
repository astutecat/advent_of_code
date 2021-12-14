use std::io::{BufRead, BufReader};
use std::{error::Error, i32};

use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let mut prev: Option<i32> = None;
    let mut num_increased = 0;
    let mut num_decreased = 0;
    let mut no_change = 0;

    let file = File::open("input.txt")?;

    let reader = BufReader::new(file);

    let parsed:Vec<i32> = reader.lines().map(|x| x.unwrap().parse::<i32>().unwrap()).collect();

    for i in parsed.windows(3) {
      let curr = i.iter().sum::<i32>();
      if let Some(p) = prev {
        if curr > p {
          num_increased += 1;
        } else if curr < p {
          num_decreased += 1;
        } else {
          no_change += 1;
        }
      }
      prev = Some(curr);
    }

    println!(
        "decreased: {}, increased: {}, no change: {}",
        num_increased, num_decreased, no_change
    );
    Ok(())
}

