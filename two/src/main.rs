use std::io::{self, Read};

fn main() {
    let mut sheet = String::new();
    let stdin = io::stdin();
    let mut stdin_open = stdin.lock();
    stdin_open.read_to_string(&mut sheet).expect("Couldn't read spreadsheet");
    let mut total = 0;
    for line in sheet.lines() {
        let nums: Vec<u32> = line.split_whitespace().map(|x| { 
            let value: u32 = x.parse().expect("Should be a number");
            return value;
        }).collect();
        println!("Nums are {:?}", nums);
        let max = nums.iter().max();
        let min = nums.iter().min();
        total += max.unwrap() - min.unwrap();
    }
    println!("Checksum is {}", total);
}
