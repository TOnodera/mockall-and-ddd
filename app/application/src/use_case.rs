use domain::{
    EmailAddress, MyError, MyResult, Repositories, User, UserFirstName, UserLastName, UserName,
    UserRepository,
};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct UseCase<'r, R>
where
    R: Repositories,
{
    user_repo: &'r R::UserRepo,
}

impl<'r, R> UseCase<'r, R>
where
    R: Repositories,
{
    pub fn new(repositories: &'r R) -> Self {
        Self {
            user_repo: repositories.user_repository(),
        }
    }

    pub fn search_users(
        &self,
        first_name: Option<&UserFirstName>,
        last_name: Option<&UserLastName>,
        email: Option<&EmailAddress>,
    ) -> Vec<User> {
        todo!()
    }

    pub fn add_user(&self, user: User) -> MyResult<()> {
        todo!()
    }

    pub fn update_user_by_email(
        &self,
        email: &EmailAddress,
        first_name: Option<UserFirstName>,
        last_name: Option<UserLastName>,
    ) -> MyResult<()> {
        todo!()
    }
}
