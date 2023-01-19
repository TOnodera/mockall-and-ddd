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
        let users = self.user_repo.list();
        let users = users
            .into_iter()
            .filter(|user| {
                if let Some(first_name) = first_name {
                    return first_name == user.name().first_name();
                }
                if let Some(last_name) = last_name {
                    return last_name == user.name().last_name();
                }
                if let Some(email) = email {
                    return email == user.email();
                }
                // どれにも当てはまらない場合はfalse
                false
            })
            .collect();
        users
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
