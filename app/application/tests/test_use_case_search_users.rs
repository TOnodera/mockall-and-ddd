use application::UseCase;
use domain::{
    EmailAddress, MyResult, Repositories, User, UserFirstName, UserLastName, UserRepository,
};

/**
// 愚直にやろうとするとこんな感じになる
// START 愚直
pub struct EmptyUserRepository;

impl UserRepository for EmptyUserRepository {
    fn list(&self) -> Vec<User> {
        vec![]
    }

    fn create(&self, user: User) -> MyResult<()> {
        unimplemented!()
    }

    fn update(&self, user: User) -> MyResult<()> {
        unimplemented!()
    }
}
pub struct EmptyRepositories {
    user_repo: EmptyUserRepository,
}
impl Repositories for EmptyRepositories {
    type UserRepo = EmptyUserRepository;

    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repo
    }
}
impl EmptyRepositories {
    pub fn new(user_repo: EmptyUserRepository) -> Self {
        Self { user_repo }
    }
}
// END 愚直
*/

#[test]
fn test_with_blank_repository() {
    let user_repo = EmptyUserRepository;
    let repositories = EmptyRepositories::new(user_repo);
    let use_case = UseCase::new(&repositories);

    assert_eq!(use_case.search_users(None, None, None), vec![]);
    assert_eq!(
        use_case.search_users(Some(&UserFirstName::new("a")), None, None),
        vec![]
    );
    assert_eq!(
        use_case.search_users(None, Some(&UserLastName::new("a")), None),
        vec![]
    );
    assert_eq!(
        use_case.search_users(None, None, Some(&EmailAddress::new("a@b"))),
        vec![]
    );
}
