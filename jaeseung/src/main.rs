mod domain;
mod utils;
mod dependency;
mod user;

use std::ops::Deref;
use utils::*;
use domain::*;

fn main() {
    loop {
        let menu = print_menu_form(&["로그인", "회원가입"]);

        let result = match menu.deref().as_str() {
            "1" => login(),
            "2" => sign_in(),
            _ => {
                println!("잘못된 입력입니다. 처음부터 다시 시도해 주세요.");
                Err("잘못된 입력입니다. 처음부터 다시 시도해 주세요.".to_string())
            }
        };

        match result {
            Ok(_) => {
                println!("로그인 성공!");
            }
            Err(msg) => {
                println!("{}\n", msg);
            }
        }
    }
}