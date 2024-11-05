use chrono::{DateTime, FixedOffset, Utc};
use super::repo;

#[derive(Debug)]
pub struct SignUpForm {
    pub email: String,
    pub password: String,
    pub username: String,
    pub created_time: DateTime<FixedOffset>,
}

impl SignUpForm {
    pub fn new(email: String, password: String, username: String) -> SignUpForm {
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
    pub fn post_user_auth(sign_up_form: &SignUpForm) {
        repo::insert_user(sign_up_form);
        let result = repo::select_user_with_email(&sign_up_form.email);
        println!("{:?}", result);
    }
}
