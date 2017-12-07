use std::collections::HashMap;
use std::io::{self, Read};
use regex::Regex;
use indextree::{Arena, NodeId};

extern crate regex;
extern crate indextree;

fn main() {
    let mut edges_string = String::new();
    let stdin = io::stdin();
    let mut stdin_open = stdin.lock();
    stdin_open.read_to_string(&mut edges_string).expect("Couldn't edges");
    //let valid_passwords: Vec<&str> = passwords.lines().filter(|word| validate_password(word)).collect();

    let tree = &mut Arena::new();
    let mut nodes: HashMap<&str, NodeId> = HashMap::new();

    let edges = Regex::new(r"^([a-z]+) \(([0-9]+)\)( -> (.*?))?$").unwrap();
    for line in edges_string.lines() {
        println!("Line is {}", line);
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

    //println!("Tree {:?}", tree);
    for node in nodes {
        if node.1.ancestors(tree).into_iter().count() == 1 {
            println!("Node {:?}", node);
        }
    }
}
