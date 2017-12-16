use std::io;

fn check_group(group: &str) -> (u32, u32) {
    let mut depth = 0;
    let mut cancelled = false;
    let mut garbage = false;
    let mut score = 0;
    let mut num_cancelled = 0;

    for char in group.chars() {
        if cancelled {
            cancelled = false;
        } else if garbage {
            match char {
                '!' => cancelled = !cancelled,
                '>' => garbage = false,
                _ => num_cancelled += 1,
            }
        } else {
            match char {
                '{' => depth += 1,
                '}' => {
                    score += depth;
                    depth -= 1; 
                },
                '<' => garbage = true,
                '!' => cancelled = !cancelled,
                ',' => (),
                _ => panic!("I don't understand {}", char),
            }
        }
    }
    return (score, num_cancelled);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    println!("{} -> {:?}", input, check_group(input.trim()));
}


#[test]
fn test_trivial_group() {
    assert_eq!(check_group("{}").0, 1);
}

#[test]
fn test_deeply_nested() {
    assert_eq!(check_group("{{{}}}").0, 6);
}

#[test]
fn test_multiple_nested() {
    assert_eq!(check_group("{{},{}}").0, 5);
}

#[test]
fn test_differnet_nestings() {
    assert_eq!(check_group("{{{},{},{{}}}}").0, 16);
}

#[test]
fn test_garbage_in_group() {
    assert_eq!(check_group("{<a>,<a>,<a>,<a>}").0, 1);
}

#[test]
fn test_garbage_nested() {
    assert_eq!(check_group("{{<ab>},{<ab>},{<ab>},{<ab>}}").0, 9);
}

#[test]
fn test_double_cancellations() {
    assert_eq!(check_group("{{<!!>},{<!!>},{<!!>},{<!!>}}").0, 9);
}

#[test]
fn test_cancelled_garbage_escaped() {
    assert_eq!(check_group("{{<a!>},{<a!>},{<a!>},{<ab>}}").0, 3);
}

#[test]
fn test_no_garbage() {
    assert_eq!(check_group("<>").1, 0);
}

#[test]
fn test_some_garbage() {
    assert_eq!(check_group("<random characters>").1, 17);
}

#[test]
fn test_multiple_garbage_statts() {
    assert_eq!(check_group("<<<<>").1, 3);
}

#[test]
fn test_cancelled_ending() {
    assert_eq!(check_group("<{!>}>").1, 2);
}

#[test]
fn test_double_cancelled_inside_garbage() {
    assert_eq!(check_group("<!!>").1, 0);
}

#[test]
fn test_triple_cancelled_inside_garbage() {
    assert_eq!(check_group("<!!!>>").1, 0);
}

#[test]
fn test_complex_garbage() {
    assert_eq!(check_group("<{o\"i!a,<{i<a>").1, 10);
}