use std::io::{self, Read};

fn main() {
    let mut instructions = String::new();
    let stdin = io::stdin();
    let mut stdin_open = stdin.lock();
    stdin_open.read_to_string(&mut instructions).expect("Couldn't read instructions");
    let mut offsets: Vec<i32> = instructions.lines().map(|offset| offset.parse().expect("Needed an integer")).collect();
    let mut pc: usize = 0;
    let mut steps = 0;
    while pc < offsets.len() {
        let instruction = offsets[pc];
        steps += 1;
        if instruction >= 3 {
            offsets[pc] -= 1 
        } else {
            offsets[pc] += 1;
        }
        //println!("Adding {} to {}", instruction, pc);
        //println!("PC is {:?}, memory is {:?}", pc, offsets);
        pc = ((pc as i32) + instruction) as usize;
    }
    println!("Escaped in {} steps", steps);
}
