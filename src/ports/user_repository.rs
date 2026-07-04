use uuid::Uuid;

use crate::domain::user::User;

pub trait UserRepository {
    fn get_user(&self, id: Uuid) -> Option<User>;
    fn get_all_users(&self) -> Vec<User>;
    fn create_user(&self, user: User) -> User;
    fn update_user(&self, user: User) -> Option<User>;
    fn delete_user(&self, id: Uuid) -> bool;
}
