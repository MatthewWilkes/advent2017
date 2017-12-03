use std::io;
use std::collections::HashMap;

// Debug = printable
// Hash, Eq and PartialEq = Hashmap
// Clone, Copy = Don't edit in place
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
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
    // A hashmap of coordinate to the value at that coord
    let mut values: HashMap<Coordinate, u32> = HashMap::new();

    // Initialise the home with 1
    values.insert(coord.clone(), 1);

    for i in 0..target_number {
        step += 1;

        let mut part_value: u32 = 0;
        for x_offset in -1..2 {
            for y_offset in -1..2 {
                let offset_coord = Coordinate {
                    x: coord.x + x_offset,
                    y: coord.y + y_offset,
                };
                let offset_value = values.get(&offset_coord);
                if offset_value.is_some() {
                    part_value += offset_value.unwrap();
                }
            }
        }
        values.insert(coord.clone(), part_value);
        if part_value > target_number {
            println!("Memory value at step {} is {}", i, values.get(&coord).unwrap());
            break;
        }
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

    let distance = coord.x.abs() + coord.y.abs();
    //println!("Manhattan distance is {}", distance);
}
