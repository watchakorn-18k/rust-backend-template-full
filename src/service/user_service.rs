use log::info;

use crate::models::User;
use crate::repository::user_repo::UserRepository;

pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub fn new(user_repository: UserRepository) -> Self {
        UserService { user_repository }
    }

    pub fn get_all(&self) -> Result<Vec<User>, String> {
        let users = self.user_repository.get_all();
        info!("users: {:#?}", users);
        match users {
            Ok(users) => Ok(users),
            Err(e) => Err(e.to_string()),
        }
    }
}
