use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let target_number: u32 = input.trim().parse().expect("Did not get an integer");
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    // The pattern of movements is right, up, left, down repeated.
    let mut part = 0;
    // The distances are 1, 1, 2, 2, 3, 3, 4, 4 etc
    let mut step = 0;
    for i in 0..target_number-1 {
        step += 1;
        // The part remainder 4 is the direction we are going
        match part % 4 {
            0 => x += 1,
            1 => y += 1,
            2 => x -= 1,
            3 => y -= 1,
            _ => println!("Huh?"),
        }
        // The number of steps to finish a part starts at 1 and goes up every 2 parts
        if step == (((part as f64) / 2.0).floor() as u32) + 1 {
            step = 0;
            part += 1;
        }
    }
    println!("Memory address {} is at {},{}", target_number,x,y);
    let distance = x.abs() + y.abs();
    println!("Manhattan distance is {}", distance);
}
