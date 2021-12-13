use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;

    let reader = BufReader::new(file);

    let mut prev_mesurement: Option<i32> = None;

    let mut num_increased = 0;
    let mut num_decreased = 0;

    for line in reader.lines() {
        if let Ok(l) = line {
            let parsed = l.parse::<i32>()?;
            match prev_mesurement {
                None => {
                    println!("{} (N/A - No Previous Measurement)", parsed);
                    prev_mesurement = Some(parsed);
                }
                Some(a) if a < parsed => {
                    num_increased += 1;
                    prev_mesurement = Some(parsed);
                    println!("{} (Increased)", parsed)
                }
                Some(a) if a >= parsed => {
                    num_decreased += 1;
                    prev_mesurement = Some(parsed);
                    println!("{} (Decreased)", parsed)
                }
                Some(_) => (),
            }
        }
    }

    println!(
        "increased total: {}\n decreased total: {}",
        num_increased, num_decreased
    );
    Ok(())
}
