use std::error::Error;

use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let mut prev: Option<u32> = None;
    let mut num_increased = 0;

    let data = fs::read_to_string("input.txt").expect("unable to read file");

    let parsed:Vec<u32> = data.lines().map(|x| x.parse::<u32>().unwrap()).collect();

    for i in parsed.windows(3) {
      let curr = i.iter().sum::<u32>();
      if let Some(p) = prev {
        if curr > p {
          num_increased += 1;
        }
      }
      prev = Some(curr);
    }

    println!("increased: {}", num_increased);
    Ok(())
}

