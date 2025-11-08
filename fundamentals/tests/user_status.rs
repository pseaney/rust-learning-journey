use fundamentals::user::User;
use fundamentals::status::Status;

#[test]
fn test_user_lifecycle() {
    let mut user = User::new("Paul", 30);
    assert_eq!(user.username, "Paul");
    assert_eq!(user.age, 30);
    matches!(user.status, Status::Pending);

    user.activate();
    matches!(user.status, Status::Active);
}