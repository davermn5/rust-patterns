use rplib::frontdoor::user_login_api;

#[test]
fn user_login() {
    let login_state = user_login_api(&true);
    assert_eq!(login_state, true);
}