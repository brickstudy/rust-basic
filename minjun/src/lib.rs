use std::io;

pub fn get_input() -> Option<String> {
    let mut buffer: String = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again.");
    };
    // trim whitespace from input
    let input: String = buffer.trim().to_owned();
    if input.is_empty() {
        None
    } else {
        Some(input)
    }
}
