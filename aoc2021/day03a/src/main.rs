use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("unable to read file");
    let parsed:Vec<i32> = data.lines().map(|str| i32::from_str_radix(&str, 2).unwrap()).collect();
    let bit_len: usize = 32 - parsed.iter().max().unwrap().leading_zeros() as usize;

    println!("Bit length: {}", bit_len);



    let s = "1101"; // binary digits

    let i = i32::from_str_radix(s, 2).expect("Not a binary number!");

    println!("{}", i);

    println!("{}", (i >> 0) & 1); // 1
    println!("{}", (i >> 1) & 1); // 0
    println!("{}", (i >> 2) & 1); // 1
    println!("{}", (i >> 3) & 1); // 1

}
