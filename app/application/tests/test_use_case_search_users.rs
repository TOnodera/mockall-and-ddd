use application::UseCase;
use domain::{
    EmailAddress, MockUserRepository, MyResult, Repositories, User, UserFirstName, UserLastName,
    UserRepository,
};

// MockAllで実装
pub struct TestRepositories {
    user_repo: MockUserRepository,
}
impl Repositories for TestRepositories {
    type UserRepo = MockUserRepository;

    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repo
    }
}
impl TestRepositories {
    pub fn new(user_repo: MockUserRepository) -> Self {
        Self { user_repo }
    }
}

#[test]
fn test_with_blank_repository() {
    let mut user_repo = MockUserRepository::new();
    // mockのlist()をクロージャで差し替える
    user_repo.expect_list().returning(|| vec![]);

    let repositories = TestRepositories::new(user_repo);
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
