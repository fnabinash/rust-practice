use modules_09::User;

#[test]
fn test_username() {
    let user: User = User::default_user();
    assert_eq!("Sam", user.username());
}

#[test]
#[should_panic(expected = "Error: Wrong password!")]
fn test_need_correct_password_to_change_username() {
    let mut user: User = User::default_user();
    user.change_username("wrong_password", "Try").unwrap();
}

#[test]
#[should_panic(expected = "Error: Please enter a new username.")]
fn test_new_username_can_not_same_as_old_username() {
    let mut user: User = User::default_user();
    user.change_username("sam", "Sam").unwrap();
}

#[test]
fn test_successful_changed_username() {
    let mut user: User = User::default_user();
    assert_eq!("Sam", user.username());

    user.change_username("sam", "Rufos").unwrap();
    assert_eq!("Rufos", user.username());
}

#[test]
#[should_panic(expected = "Error: Wrong password!")]
fn test_need_correct_password_to_change_password() {
    let mut user: User = User::default_user();
    user.change_password("wrong_password", "Try").unwrap();
}

#[test]
#[should_panic(expected = "Error: Please enter a new password.")]
fn test_new_password_can_not_same_as_old_password() {
    let mut user: User = User::default_user();
    user.change_password("sam", "sam").unwrap();
}
