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
        for (i, num_1) in nums.iter().enumerate() {
            for num_2 in nums.iter().skip(i + 1) {
                if num_1 % num_2 == 0 || num_2 % num_1 == 0 {
                    let max = num_1.max(num_2);
                    let min = num_1.min(num_2);
                    total += max / min;
                }
            }
        }
    }
    println!("Checksum is {}", total);
}
