use std::collections::HashMap;
use std::io::{self, Read};
use regex::Regex;
use indextree::{Arena, NodeId};

extern crate regex;
extern crate indextree;

fn weight(node: NodeId, tree: &Arena<&str>, weights: &HashMap<&str, i32>) -> i32 {
    let mut weight = 0;
    for child in node.descendants(tree) {
        weight += weights.get(tree[child].data).unwrap();
    }
    return weight;
}

fn main() {
    let mut edges_string = String::new();
    let stdin = io::stdin();
    let mut stdin_open = stdin.lock();
    stdin_open.read_to_string(&mut edges_string).expect("Couldn't edges");
    //let valid_passwords: Vec<&str> = passwords.lines().filter(|word| validate_password(word)).collect();

    let tree = &mut Arena::new();
    let mut nodes: HashMap<&str, NodeId> = HashMap::new();
    let mut weights: HashMap<&str, i32> = HashMap::new();

    let edges = Regex::new(r"^([a-z]+) \(([0-9]+)\)( -> (.*?))?$").unwrap();
    for line in edges_string.lines() {
        let captured = edges.captures(line).expect("Could not parse line");
        let head = captured.get(1).unwrap().as_str();
        let weight = captured.get(2).unwrap().as_str();
        let children = captured.get(4);
        let mut node;
        if nodes.contains_key(head) {
            node = *nodes.get(head).unwrap();
        } else {
            node = tree.new_node(head);
            nodes.insert(head, node);
        }
        weights.insert(head, weight.parse().unwrap());
        let mut child_nodes: Vec<&str> = Vec::new();
        if children.is_some() {
            for child in children.unwrap().as_str().split(", ") {    
                let child_node;    
                if nodes.contains_key(child) {
                    child_node = *nodes.get(child).unwrap();
                } else {
                    child_node = tree.new_node(child);
                    nodes.insert(child, child_node);
                }
                node.append(child_node, tree);
            }
        };
        //tree.insert(head, child_nodes);
        //println!("Captured is {:?}", captured);
    }

    println!("Tree {:?}", tree);
    let mut head = None;

    for node in nodes {
        if node.1.ancestors(tree).into_iter().count() == 1 {
            head = Some(node.1);
            println!("Head is {:?}", node.0);
            break;
        }
    }
    if head.is_some() {
        let mut current = head.unwrap();
        let mut previous_diff: i32 = 0;
        loop {
            let mut child_weights: Vec<i32> = Vec::new();
            println!("Node {:?} has inherent weight {}", tree[current].data, weights.get(tree[current].data).unwrap());
            for node in current.children(tree) {
                let node_weight = weight(node, tree, &weights);
                println!("Node {:?} has total weight {}", tree[node].data, node_weight);
                child_weights.push(node_weight);
            }
            let max = child_weights.iter().max().unwrap();
            let min = child_weights.iter().min().unwrap();
            
            let num_maxes = child_weights.iter().enumerate().filter(|x| x.1 == max).count();
            let num_mins = child_weights.iter().enumerate().filter(|x| x.1 == min).count();
 
            if num_maxes == 1 {
                let mut maxes = child_weights.iter().enumerate().filter(|x| x.1 == max);
                let child_index = maxes.next().unwrap().0;
                previous_diff = *min - *max;
                current = current.children(tree).into_iter().skip(child_index).next().unwrap();
            } else if num_mins == 1 {
                let mut mins = child_weights.iter().enumerate().filter(|x| x.1 == min);
                let child_index = mins.next().unwrap().0;
                previous_diff = *max - *min;
                current = current.children(tree).into_iter().skip(child_index).next().unwrap();
            } else {
                println!("Previous was {}", previous_diff);
                println!("Inherent weight should be {}", *weights.get(tree[current].data).unwrap() + previous_diff);
                break;
            }
        }
    }
}
