use mysql::params;

use crate::user::User;

use super::UserRepository;

const DB_USER_NAME: &'static str = "root";
const DB_PASSWORD: &'static str = "1234";
const DB_IP: &'static str = "localhost";
const DB_PORT: u16 = 3306_u16;
const DB_NAME: &'static str = "my_schema";

pub struct UserDbRepository;

impl UserRepository for UserDbRepository {
    fn find_user_by_id(&self, id: &Box<String>) -> Result<Option<User>, String> {
        // TODO : 커넥션 static 하게
        let db_url = format!(
            "mysql://{}:{}@{}:{}/{}",
            DB_USER_NAME, DB_PASSWORD, DB_IP, DB_PORT, DB_NAME
        );
        let pool = mysql::Pool::new(db_url).expect("연결 실패!");

        let query = "select * from user where id=:id";

        let results = pool
            .prep_exec(query, mysql::params! { "id" => id.as_str() })
            .expect("쿼리 실패");

        let row = results.last().map(|row| row.unwrap());

        match row {
            Some(row) => {
                let (id, password): (String, String) = mysql::from_row(row);
                Ok(Some(User { id, password }))
            }
            None => Ok(None),
        }
    }

    fn save_user(&self, user: User) -> Result<(), String> {
        let db_url = format!(
            "mysql://{}:{}@{}:{}/{}",
            DB_USER_NAME, DB_PASSWORD, DB_IP, DB_PORT, DB_NAME
        );
        let pool = mysql::Pool::new(db_url).expect("연결 실패!");

        let query = "insert into user (id, password) values (:id, :password)";

        pool.prep_exec(
            query,
            mysql::params! { "id" => user.id.as_str(), "password" => user.password.as_str() },
        )
        .expect("쿼리 실패");

        Ok(())
    }
}
