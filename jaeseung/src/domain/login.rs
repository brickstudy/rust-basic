use crate::utils::get_input;

pub fn login() -> Result<(), String> {
    let id = get_input("ID 를 입력해 주세요");
    let password = get_input("비밀번호를 입력해 주세요");

    let user_opt = USER_REPO.find_user_by_id(&id)?;

    match user_opt {
        Some(user) if user.password.eq(password.deref()) => Ok(()),
        _ => Err("id, password 를 다시 확인해 주세요.".to_string())
    }
}