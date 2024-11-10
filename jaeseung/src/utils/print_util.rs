use std::io::{self, Write};

pub fn print_menu_form(menus: &[&str]) -> Box<String> {
    println!("==================");
    for (index, menu) in menus.iter().enumerate() {
        println!("{}. {}", index + 1, menu);
    }
    println!("==================");
    get_input("입력")
}

pub fn get_input(order: &str) -> Box<String> {
    print!("{} : ", order);
    io::stdout().flush().expect("Flush 실패!");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("read_line Error 발생!");
    Box::from(input.trim().to_string())
}