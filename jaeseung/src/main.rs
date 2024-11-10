use std::fs::OpenOptions;
use std::io;
use std::io::{Read, Write};
use std::ops::Deref;

struct User {
    id: String,
    password: String
}

// TODO : 함수 모듈화하기
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

fn login() -> Result<(), String> {
    let id = get_input("ID 를 입력해 주세요");
    let password = get_input("비밀번호를 입력해 주세요");

    let user_opt = find_user_by_id(&id)?;

    match user_opt {
        Some(user) if user.password.eq(password.deref()) => Ok(()),
        _ => Err("id, password 를 다시 확인해 주세요.".to_string())
    }
}

// TODO : 트레이트로
fn find_user_by_id(id: &Box<String>) -> Result<Option<User>, String> {
    let mut file = OpenOptions::new().read(true).open("./users.csv").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("read file Error 발생!");

    // TODO : 함수형으로 작성
    for content in contents.split("\n") {
        let mut row = content.split(",").collect::<Vec<&str>>();
        if row.len() < 2 { break; }

        let exist_id = row.get(0).unwrap();
        let password = row.get(1).unwrap();

        if id.deref() == exist_id {
            return Ok(Some(User { id: exist_id.to_string(), password: password.to_string()}));
        }
    }

    Ok(None)
}

fn sign_in() -> Result<(), String> {
    let id = get_input("ID 를 입력해 주세요");
    let user = find_user_by_id(&id)?;

    match user {
        Some(_) => Err("중복된 id 입니다. 처음부터 다시 시도해 주세요.".to_string()),
        None => {
            let password = get_input("비밀번호를 입력해 주세요");

            let password_check = get_input("비밀번호를 다시 입력해 주세요");

            if password == password_check {
                save_user(User { id: id.to_string(), password: password.to_string() })
            } else {
                Err("비밀번호가 다릅니다. 처음부터 다시 시도해 주세요.".to_string())
            }
        }
    }
}

// TODO : 트레이트로
fn save_user(user: User) -> Result<(), String> {
    let mut file = OpenOptions::new().write(true).append(true).open("./users.csv").unwrap();
    let content = format!("{},{}\n", user.id, user.password);

    file.write(content.as_bytes()).expect("write file Error 발생!");
    Ok(())
}

fn print_menu_form(menus: &[&str]) -> Box<String> {
    println!("==================");
    for (index, menu) in menus.iter().enumerate() {
        println!("{}. {}", index + 1, menu);
    }
    println!("==================");
    get_input("입력")
}

fn get_input(order: &str) -> Box<String> {
    print!("{} : ", order);
    io::stdout().flush().expect("Flush 실패!");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("read_line Error 발생!");
    Box::from(input.trim().to_string())
}