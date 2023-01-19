mod fixture;

use application::UseCase;
use domain::{
    EmailAddress, MockUserRepository, MyResult, Repositories, User, UserFirstName, UserLastName,
    UserRepository,
};
use fixture::UserFixture;

use crate::fixture::{EmailAddressFixture, UserFirstNameFixture};

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

#[test]
fn test_with_3user_repository() {
    let mut user_repo = MockUserRepository::new();
    user_repo
        .expect_list()
        .returning(|| vec![User::fx1(), User::fx2(), User::fx3()]);

    let repositories = TestRepositories::new(user_repo);
    let use_case = UseCase::new(&repositories);

    assert_eq!(use_case.search_users(None, None, None), vec![]);
    assert_eq!(
        use_case.search_users(Some(&UserFirstName::fx1()), None, None),
        vec![User::fx1()]
    );
    assert_eq!(
        use_case.search_users(Some(&UserFirstName::fx2()), None, None),
        vec![User::fx2(), User::fx3()]
    );
    assert_eq!(
        use_case.search_users(None, None, Some(&EmailAddress::fx1())),
        vec![User::fx1()]
    );
    assert_eq!(
        use_case.search_users(
            Some(&UserFirstName::fx2()),
            None,
            Some(&EmailAddress::fx2())
        ),
        vec![User::fx2()]
    );
}
