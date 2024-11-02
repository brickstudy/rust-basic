use std::io;

fn get_input() -> Option<String> {
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

fn show() {
    println!("===== Login Service ====");
    println!("1. Create User.");
    println!("2. Login User.");
}

fn user_input() {
    loop {
        show();
        let input: Option<String> = get_input();
        match input.as_deref() {
            Some("1") => println!("111111"),
            Some("2") => println!("222222"),
            _ => break,
        }
    }
}

fn main() {
    user_input();
}
