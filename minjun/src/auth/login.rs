#[derive(Debug)]
pub struct LogInForm {
    email: String,
    password: String,
}

impl LogInForm {
    pub fn new(email: String, password: String) -> LogInForm {
        LogInForm {
            email,
            password,
        }
    }
}