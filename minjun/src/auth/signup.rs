use chrono::{DateTime, FixedOffset, Utc};


#[derive(Debug)]
pub struct SignUpForm {
    email: String,
    password: String,
    username: String,
    created_time: DateTime<FixedOffset>,
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
}
