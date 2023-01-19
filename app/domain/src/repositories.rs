use crate::user::user_repository::UserRepository;

pub trait Repositories {
    type UserRepo: UserRepository;
    fn user_repository(&self) -> &Self::UserRepo;
}
