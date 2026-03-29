pub fn user_login_api(has_token: &bool) -> bool {
    login(&has_token)
}

pub fn service_login_api(has_token: &bool) -> bool {
    login(&has_token)
}

fn login(has_token: &bool) -> bool {
    *has_token
}

#[cfg(test)]
mod access {
    use super::*;

    #[test]
    fn is_logged_in() {
        assert_eq!(login(&true), true)
    }
}