use minjun::get_mysql_conn;
use mysql::prelude::Queryable;
use mysql::params;

use super::signup::SignUpForm;

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
    let mut conn = get_mysql_conn().expect("Failed to connect databases");
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
    let mut conn = get_mysql_conn().expect("Failed to connect databases");
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