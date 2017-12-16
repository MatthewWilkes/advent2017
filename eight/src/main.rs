use std::collections::HashMap;
use std::io::{self, Read};

fn compare(lhs: &i32, comparison: &str, rhs: &i32) -> bool {
    match comparison {
        "<" => lhs < rhs,
        ">" => lhs > rhs,
        "==" => lhs == rhs,
        "!=" => lhs != rhs,
        "<=" => lhs <= rhs,
        ">=" => lhs >= rhs,
        _ => panic!("Unknown comparitor {}", comparison)
    }
}

fn main() {    
    let mut instructions = String::new();
    let mut registers: HashMap<&str, i32> = HashMap::new();
    let stdin = io::stdin();
    let mut stdin_open = stdin.lock();
    stdin_open.read_to_string(&mut instructions).expect("Couldn't read instruction input");

    let mut max_seen = 0;

    for instruction in instructions.lines() {
        let mut parts = instruction.split_whitespace();
        let register = parts.next().expect("Couldn't extract register");
        let operator = parts.next().expect("Couldn't extract operator");
        assert!(operator == "inc" || operator == "dec");
        let value: i32 = parts.next().expect("Couldn't extract value").parse().expect("Value was not numeric");
        assert_eq!(Some("if"), parts.next());
        let condition_register = parts.next().expect("Couldn't extract register");
        let condition = parts.next().expect("Couldn't extract condition operator");
        let condition_value: i32 = parts.next().expect("Couldn't extract condition value").parse().expect("Value was not numeric");

        
        let condition_register_value = registers.get(condition_register).unwrap_or(&0).clone();
        let condition_satisfied = compare(&condition_register_value, condition, &condition_value);
        if condition_satisfied {
            let mut register_value = registers.get(register).cloned().unwrap_or(0);
            match operator {
                "inc" => register_value += value,
                "dec" => register_value -= value,
                _ => panic!("Unknown operator {}", operator),
            }
            registers.insert(register.clone(), register_value);
        }
        let current_max = registers.values().max().cloned().unwrap_or(0);
        if current_max > max_seen {
            max_seen = current_max;
        }
    }
    let mut register_pairs: Vec<(&&str, &i32)> = registers.iter().collect();
    register_pairs.sort_by_key(|val| val.1);
    for pair in register_pairs {
        println!("{} is {}", pair.0, pair.1);
    }
    println!("Largest ever was {}", max_seen);
}
