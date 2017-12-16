use std::io;

fn check_group(group: &str) -> u32 {
    let mut depth = 0;
    let mut cancelled = false;
    let mut garbage = false;
    let mut score = 0;

    for char in group.chars() {
        if cancelled {
            cancelled = false;
        } else if garbage {
            match char {
                '!' => cancelled = !cancelled,
                '>' => garbage = false,
                _ => (),
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
    return score;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    println!("{} -> {}", input, check_group(input.trim()));
}


#[test]
fn test_trivial_group() {
    assert_eq!(check_group("{}"), 1);
}

#[test]
fn test_deeply_nested() {
    assert_eq!(check_group("{{{}}}"), 6);
}

#[test]
fn test_multiple_nested() {
    assert_eq!(check_group("{{},{}}"), 5);
}

#[test]
fn test_differnet_nestings() {
    assert_eq!(check_group("{{{},{},{{}}}}"), 16);
}

#[test]
fn test_garbage_in_group() {
    assert_eq!(check_group("{<a>,<a>,<a>,<a>}"), 1);
}

#[test]
fn test_garbage_nested() {
    assert_eq!(check_group("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
}

#[test]
fn test_double_cancellations() {
    assert_eq!(check_group("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
}

#[test]
fn test_cancelled_garbage_escaped() {
    assert_eq!(check_group("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
}
