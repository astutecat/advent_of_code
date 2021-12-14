use std::fs;

fn main() {

    let mut h_pos:i32 = 0;
    let mut depth:i32 = 0;

    let data = fs::read_to_string("input.txt").expect("unable to read file");
    let parsed:Vec<(&str, i32)> = data.lines().map(|x| {
        let mut split = x.split(" ");
        let direction = split.next().unwrap();
        let amount = split.next().unwrap().parse::<i32>().unwrap();
        (direction, amount)
    }).collect();

    for i in parsed {
        match i {
            ("forward", i) => h_pos += i,
            ("down", i) => depth += i,
            ("up", i) => depth -= i,
            (_, _) => ()
        }
    }
    println!("h pos: {}, depth: {}", h_pos, depth);
    let sol = h_pos * depth;
    println!("multiplied: {}", sol);
}
