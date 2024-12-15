use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::ops::Deref;

use crate::user::User;

use super::UserRepository;

pub struct UserFileRepository;

impl UserRepository for UserFileRepository {
    fn find_user_by_id(&self, id: &Box<String>) -> Result<Option<User>, String> {
        let mut file = OpenOptions::new().read(true).open("src/users.csv").unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("read file Error 발생!");

        // TODO : 함수형으로 작성
        for content in contents.split("\n") {
            let row = content.split(",").collect::<Vec<&str>>();
            if row.len() < 2 {
                break;
            }

            let exist_id = row.get(0).unwrap();
            let password = row.get(1).unwrap();

            if id.deref() == exist_id {
                return Ok(Some(User {
                    id: exist_id.to_string(),
                    password: password.to_string(),
                }));
            }
        }

        Ok(None)
    }

    fn save_user(&self, user: User) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("src/users.csv")
            .unwrap();
        let content = format!("{},{}\n", user.id, user.password);

        file.write(content.as_bytes())
            .expect("write file Error 발생!");
        Ok(())
    }
}
