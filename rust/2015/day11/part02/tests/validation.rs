use part01::{password_rules, AlphaNum};

#[test]
fn test_validation() {
    let password: AlphaNum = "hijklmmn".parse().unwrap();
    assert!(password_rules::valid_password(&password.get_alpha()) == false);

    let password: AlphaNum = "abbceffg".parse().unwrap();
    assert!(password_rules::valid_password(&password.get_alpha()) == false);

    let password: AlphaNum = "abbcegjk".parse().unwrap();
    assert!(password_rules::valid_password(&password.get_alpha()) == false);

    let mut password: AlphaNum = "abcdefgh".parse().unwrap();
    get_next_password(&mut password);
    assert_eq!(password.get_alpha(), "abcdffaa");

    let mut password: AlphaNum = "ghijklmn".parse().unwrap();
    get_next_password(&mut password);
    assert_eq!(password.get_alpha(), "ghjaabcc");
}

fn get_next_password(password: &mut AlphaNum) {
    while !password_rules::valid_password(password.get_alpha()) {
        password.increment();
    }
}
