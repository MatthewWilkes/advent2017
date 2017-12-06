use std::collections::HashMap;

fn redistribute(memory: &Vec<u32>) -> Vec<u32>{
   let mut tempvector = memory.clone();
   let max_value = memory.iter().max().expect("Can't order memory usage");
   let max_position = memory.iter().enumerate().find(|x| x.1==max_value).expect("Can't find max").0;
   //println!("Max value {:?}, position {:?}", max_value, max_position);
   let mut current_position = max_position;
   let mut to_allocate = tempvector[max_position];
   tempvector[max_position] = 0;
   for i in 0..to_allocate {
       current_position += 1;
       if current_position >= tempvector.len() {
           current_position = 0;
       }
       tempvector[current_position] += 1;
   }
   return tempvector;
}

fn main() {
    let initial: Vec<u32> = vec![10, 3, 15, 10, 5, 15, 5, 15, 9, 2, 5, 8, 5, 2, 3, 6];
    //let initial: Vec<u32> = vec![0, 2, 7 ,0];
    let mut seen: HashMap<Vec<u32>, u32> = HashMap::new();
    let mut current = initial;
    let mut steps = 0;
    while !seen.contains_key(&current) {
        seen.insert(current.clone(), steps);
        current = redistribute(&current);
        steps += 1;
        println!("Seen {:?} states", seen.len());
    }
    println!("redistributed {:?} times, duplicate is {:?}", steps, current);
    println!("Steps taken to see {:?} again is {}", current, steps - seen.get(&current).unwrap());
    println!("Hello, world!");
}
