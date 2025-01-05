pub mod signup;
pub mod login;
pub mod input;
pub mod repo;

use signup::SignUpForm;
use login::LogInForm;
use input::{get_email, get_password, get_username, get_command};

fn sign_up() {
    let email: String = get_email();
    let password: String = get_password();
    let username: String = get_username();
    let sign_up_form: SignUpForm = SignUpForm::new(email, password, username);
    SignUpForm::post_user_auth(&sign_up_form);
    println!("{:?}", sign_up_form);
}

fn log_in() {
    let email: String = get_email();
    let password: String = get_password();
    let log_in_form: LogInForm = LogInForm::new(email, password);
    println!("{:?}", log_in_form);
}

pub fn run() {
    loop {
        let input: Option<String> = get_command();
        match input.as_deref() {
            Some("1") => sign_up(),
            Some("2") => log_in(),
            _ => break,
        }
    }
}