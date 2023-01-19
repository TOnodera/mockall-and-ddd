use super::User;
use crate::error::MyResult;
use mockall;


#[mockall::automock]
pub trait UserRepository {
    fn list(&self) -> Vec<User>;
    fn create(&self, user: User) -> MyResult<()>;
    fn update(&self, user: User) -> MyResult<()>;
}
