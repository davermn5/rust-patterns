use rplib::frontdoor::service_login_api;

//INTEGRATION TESTS
#[test]
fn system_login() {
    let login_state = service_login_api(&true);
    assert_eq!(login_state, true);
}
