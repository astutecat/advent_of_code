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

     for line in reader.lines() {
         if let Ok(l) = line {
             let parsed = l.parse::<i32>()?;
             if window_a.len() < 3 {
                 println!("window_a length is {}, pushing {}", window_a.len(), parsed);
                 window_a.push(parsed)
             } else if window_b.len() < 3 {
                 println!("window_b length is {}, pushing {}", window_b.len(), parsed);
                 window_b.push(parsed)
             } 

             if window_a.len() == 3 && window_b.len() == 3 {
                 println!("window a and b length is 3");
                 println!("Summing window a: {}", window_a.join(","));
                 let a_sum: i32 = window_a.iter().sum();
                 println!("Summing window b: {}", window_b.join(","));
                 let b_sum: i32 = window_b.iter().sum();
                 if a_sum < b_sum {
                     num_increased += 1;
                 } else if a_sum > b_sum {
                     num_decreased += 1;
                 } else if a_sum == b_sum {
                     no_change += 1;
                 }

                 window_a = window_a.clone();
                 window_b.clear();
             }
         }
     }
    

    println!(
        "decreased: {}, increased: {}, no change: {}",
        num_increased, num_decreased, no_change
    );
    Ok(())
}

