use std::io;

fn main() {
    // Create a new string and read stdin to it
    let mut captcha = String::new();
    io::stdin().read_line(&mut captcha).expect("Failed to read captcha");
    // Trim off the newline at the end
    captcha = captcha.trim().to_string();
    let skip_num = captcha.len() / 2;
    // Create a clone of the string so we can manipulate it
    // Then reduce the string to graphemes, skip the appropriate number
    // We now have an iterable of chars rotated some amount
    // This gives us (3, 4, 5, 0, 1, 2)
    let offset_captcha = captcha.chars().skip(skip_num).chain(captcha.chars().take(skip_num));

    let mut total = 0;

    // Zip the captcha and the offset together to see the pairs
    // we should compare
    for n in captcha.chars().zip(offset_captcha) {
        // The tuple items are equal, we have a match
        if n.0 == n.1 {
            // Convert the former into an Option<u32>
            let digit = n.0.to_digit(10);
            // If Option is satisfied, unwrap and add to total
            if digit.is_some() {
                total += digit.unwrap();
            }
        }
    };

    // print the total
    println!("{}", total);
}