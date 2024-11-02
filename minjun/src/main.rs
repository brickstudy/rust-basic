use std::io;
use chrono::{DateTime, FixedOffset, Utc};


#[derive(Debug)]
struct SignUpForm {
    email: String,
    password: String,
    username: String,
    created_time: DateTime<FixedOffset>,
}

impl SignUpForm {
    fn new(email: String, password: String, username: String) -> SignUpForm {
        // set created_time on KST
        let kst_offset = FixedOffset::east_opt(9 * 60 * 60).unwrap();
        let created_time = Utc::now().with_timezone(&kst_offset);

        SignUpForm {
            email,
            password,
            username,
            created_time,
        }
    }
}

#[derive(Debug)]
struct LogInForm {
    email: String,
    password: String,
}

impl LogInForm {
    fn new(email: String, password: String) -> LogInForm {
        LogInForm {
            email,
            password,
        }
    }
}


fn get_email() -> String {
    println!("Please enter the email.\nEmail:");
    // TODO : Email 검증하는 로직 추가
    let email = get_input();
    email.unwrap()
}

fn get_password() -> String {
    println!("Please enter the password.\nPassword:");
    // TODO : Password 검증하는 로직 추가
    let password = get_input();
    password.unwrap()
}

fn get_username() -> String {
    println!("Please enter the username.\nUsername:");
    // TODO : username 검증하는 로직 추가(중복 여부?)
    let username = get_input();
    username.unwrap()
}

fn sign_up() {
    let email: String = get_email();
    let password: String = get_password();
    let username: String = get_username();
    let sign_up_form: SignUpForm = SignUpForm::new(email, password, username);
    println!("{:?}", sign_up_form);
}

fn log_in() {
    let email: String = get_email();
    let password: String = get_password();
    let log_in_form: LogInForm = LogInForm::new(email, password);
    println!("{:?}", log_in_form);
}

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
            Some("1") => sign_up(),
            Some("2") => log_in(),
            _ => break,
        }
    }
}

fn main() {
    user_input();
}
