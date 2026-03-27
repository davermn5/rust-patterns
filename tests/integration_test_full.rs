use rplib::frontdoor::user_login_api;
use rplib::frontdoor::service_login_api;

#[test]
#[ignore]
fn user_login() {
    let login_state = user_login_api(&true);
    assert_eq!(login_state, true);
}

#[test]
#[ignore]
fn system_login() {
    let login_state = service_login_api(&true);
    assert_eq!(login_state, true);
}