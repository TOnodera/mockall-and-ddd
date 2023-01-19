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
        if first_name == None && last_name == None && email == None {
            return vec![];
        }
        let users = self.user_repo.list();
        let users = users
            .into_iter()
            .filter(|user| {
                first_name
                    .map(|f_name| user.name().first_name().eq(f_name))
                    .unwrap_or_else(|| true)
                    && last_name
                        .map(|l_name| user.name().last_name().eq(l_name))
                        .unwrap_or_else(|| true)
                    && email.map(|em| em == user.email()).unwrap_or_else(|| true)
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
