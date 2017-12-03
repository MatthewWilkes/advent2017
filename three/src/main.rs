use std::io;

struct Coordinate {
    pub x: i32,
    pub y: i32,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let target_number: u32 = input.trim().parse().expect("Did not get an integer");
    let mut coord = Coordinate{
        x: 0,
        y: 0
    };
    // The pattern of movements is right, up, left, down repeated.
    let mut part = 0;
    // The distances are 1, 1, 2, 2, 3, 3, 4, 4 etc
    let mut step = 0;
    for i in 0..target_number-1 {
        step += 1;
        // The part remainder 4 is the direction we are going
        match part % 4 {
            0 => coord.x += 1,
            1 => coord.y += 1,
            2 => coord.x -= 1,
            3 => coord.y -= 1,
            _ => println!("Huh?"),
        }
        // The number of steps to finish a part starts at 1 and goes up every 2 parts
        if step == (((part as f64) / 2.0).floor() as u32) + 1 {
            step = 0;
            part += 1;
        }
    }
    println!("Memory address {} is at {},{}", target_number, coord.x, coord.y);
    let distance = coord.x.abs() + coord.y.abs();
    println!("Manhattan distance is {}", distance);
}
