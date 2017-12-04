use std::io::{self, Read};

fn validate_password(passphrase: &str) -> bool {
    let mut words: Vec<Vec<char>> = Vec::new();
    for word in passphrase.split_whitespace() {
        let mut word: Vec<char> = word.chars().collect();
        word.sort_by(|a, b| a.cmp(b));
        if words.contains(&word) {
            return false;
        } else {
            words.push(word);
        }
    }
    return true;
}

fn main() {
    let mut passwords = String::new();
    let stdin = io::stdin();
    let mut stdin_open = stdin.lock();
    stdin_open.read_to_string(&mut passwords).expect("Couldn't read passwords");
    let valid_passwords: Vec<&str> = passwords.lines().filter(|word| validate_password(word)).collect();
    println!("Number valid is {}", valid_passwords.len());
}
