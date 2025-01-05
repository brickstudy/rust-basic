use minjun::get_input;

pub fn get_command() -> Option<String> {
    println!("===== Login Service ====");
    println!("1. Create User.");
    println!("2. Login User.");
    let command = get_input();
    command
}

pub fn get_email() -> String {
    println!("Please enter the email.\nEmail:");
    // TODO : Email 검증하는 로직 추가
    let email = get_input();
    email.unwrap()
}

pub fn get_password() -> String {
    println!("Please enter the password.\nPassword:");
    // TODO : Password 검증하는 로직 추가
    let password = get_input();
    password.unwrap()
}

pub fn get_username() -> String {
    println!("Please enter the username.\nUsername:");
    // TODO : username 검증하는 로직 추가(중복 여부?)
    let username = get_input();
    username.unwrap()
}
