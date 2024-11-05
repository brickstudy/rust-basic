use minjun::get_mysql_conn;
use mysql::prelude::Queryable;
use mysql::params;

use super::signup::SignUpForm;

const DB_CONNECTION_ERROR: &str = "Failed to connect database";


#[derive(Debug)]
pub struct Users {
    pub email: String,
    pub password: String,
    pub username: Option<String>,
}

impl Users {
    pub fn new(email: String, password: String, username: Option<String>) -> Users{
        Users {
            email: email,
            password: password,
            username: username,
        }
    }
}


pub fn insert_user(sign_up_form: &SignUpForm) {
    let mut conn = get_mysql_conn().expect(DB_CONNECTION_ERROR);
    conn.exec_drop(
        "INSERT INTO minjun_users (email, password, username, created_time) 
                VALUES (:email, :password, :username, :created_time)",
        params! {
            "email" => &sign_up_form.email,
            "password" => &sign_up_form.password,
            "username" => &sign_up_form.username,
            "created_time" => &sign_up_form.created_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        },
    ).expect("Failed to insert user into the database");
}

pub fn select_user_with_email(email: &String) -> Option<Users>{
    let mut conn = get_mysql_conn().expect(DB_CONNECTION_ERROR);

    let result = conn.exec_first(
        "SELECT email, password, username from minjun_users
                WHERE email = :email",
        params! {
            "email" => email,
        },
    ).ok().flatten();

    result.map(|(email, password, username)| Users {
        email,
        password,
        username,
    })
}

pub fn delete_user_with_email(email: &String) -> Option<String>{ 
    let mut conn = get_mysql_conn().expect(DB_CONNECTION_ERROR);
    // 사용자 있는지 확인
    let result = conn.exec_first(
        "SELECT email FROM minjun_users WHERE email = :email",
        params! {
            "email" => &email
        }
    ).expect("Failed to query user from the database.");

    if let Some(existing_email) = result {
        conn.exec_drop(
            "DELETE FROM minjun_users WHERE email = :email",
            params! {
                "email" => &existing_email,
            }
        ).expect("Failed to delete user from the database");
        
        return Some(existing_email);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_crud_user() {
        // mock
        let email = "test@example.com";

        let sign_up_form: SignUpForm = SignUpForm::new(
            email.to_string(),
            "123455".to_string(),
            "test_user".to_string()
        );

        insert_user(&sign_up_form);

        let result = select_user_with_email(&email.to_string());
        let user = result.unwrap();
        assert_eq!(user.email, sign_up_form.email);
        assert_eq!(user.password, sign_up_form.password);
        assert_eq!(user.username.as_deref(), Some(sign_up_form.username.as_str()));

        // delete 
        let delete_email = delete_user_with_email(&email.to_string());
        assert_eq!(delete_email, Some(email.to_string()));
    }
}