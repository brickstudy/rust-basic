use crate::user::User;

pub trait UserRepository {
    fn find_user_by_id(&self, id: &Box<String>) -> Result<Option<User>, String>;
    fn save_user(&self, user: User) -> Result<(), String>;
}
