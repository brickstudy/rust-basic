use crate::dependency::USER_REPO;
use crate::get_input;
use crate::user::{User, UserRepository};

pub fn sign_in() -> Result<(), String> {
    let id = get_input("ID 를 입력해 주세요");
    let user = USER_REPO.find_user_by_id(&id)?;

    match user {
        Some(_) => Err("중복된 id 입니다. 처음부터 다시 시도해 주세요.".to_string()),
        None => {
            let password = get_input("비밀번호를 입력해 주세요");

            let password_check = get_input("비밀번호를 다시 입력해 주세요");

            if password == password_check {
                USER_REPO.save_user(User {
                    id: id.to_string(),
                    password: password.to_string(),
                })
            } else {
                Err("비밀번호가 다릅니다. 처음부터 다시 시도해 주세요.".to_string())
            }
        }
    }
}
