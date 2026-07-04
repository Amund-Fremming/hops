use uuid::Uuid;

use crate::domain::user::User;
use crate::ports::user_repository::UserRepository;

pub struct PostgresUserRepository;

impl PostgresUserRepository {
    pub fn new() -> Self {
        Self
    }
}

impl UserRepository for PostgresUserRepository {
    fn get_user(&self, _id: Uuid) -> Option<User> {
        todo!()
    }

    fn get_all_users(&self) -> Vec<User> {
        todo!()
    }

    fn create_user(&self, _user: User) -> User {
        todo!()
    }

    fn update_user(&self, _user: User) -> Option<User> {
        todo!()
    }

    fn delete_user(&self, _id: Uuid) -> bool {
        todo!()
    }
}
