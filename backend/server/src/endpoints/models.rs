use serde::{Deserialize, Serialize};

use crate::request_validator::RequestValidator;

#[derive(Deserialize)]
pub(crate) struct RegisterRequest {
    pub first_name: String,
    pub login: String,
    pub password: String,
}

impl RegisterRequest {
    pub fn is_valid(&self) -> bool {
        RequestValidator::does_first_name_meet_requirements(&self.first_name)
            && RequestValidator::does_login_meet_requirements(&self.login)
            && RequestValidator::does_password_meet_requirements(&self.password)
    }

    pub fn trim_inputs(&mut self) {
        self.first_name = self.first_name.trim().into();
        self.login = self.login.trim().into();
        self.password = self.password.trim().into();
    }
}

#[derive(Deserialize)]
pub(crate) struct LoginRequest {
    pub login: String,
    pub password: String,
}

impl LoginRequest {
    pub fn trim_inputs(&mut self) {
        self.login = self.login.trim().into();
        self.password = self.password.trim().into();
    }
}
#[derive(Serialize)]
pub(crate) struct Response {
    pub message: String,
}

#[derive(Serialize)]
pub(crate) struct UserResponse {
    pub first_name: String,
}
impl UserResponse {
    pub fn new(first_name: String) -> Self {
        Self { first_name }
    }
}

#[cfg(test)]
mod test {
    use crate::endpoints::models::RegisterRequest;
    fn get_valid_request() -> RegisterRequest {
        static VALID_PASSWORD: &str = "aw4vrtaa[wi93@sfe";
        static VALID_FIRST_NAME: &str = "Edd";
        static VALID_LOGIN: &str = "edd11001";
        RegisterRequest {
            first_name: VALID_FIRST_NAME.into(),
            login: VALID_LOGIN.into(),
            password: VALID_PASSWORD.into(),
        }
    }
    #[test]
    fn given_valid_request_is_valid_returns_true() {
        let valid_request = get_valid_request();
        assert_eq!(valid_request.is_valid(), true)
    }

    #[test]
    fn given_request_with_invalid_password_is_valid_returns_false() {
        static INVALID_PASSWORD: &str = "1234";
        let invalid_request = RegisterRequest {
            password: INVALID_PASSWORD.into(),
            ..get_valid_request()
        };
        assert_eq!(invalid_request.is_valid(), false)
    }

    #[test]
    fn given_request_with_invalid_first_name_is_valid_returns_false() {
        static INVALID_FIRST_NAME: &str = "martiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiin";
        let invalid_request = RegisterRequest {
            first_name: INVALID_FIRST_NAME.into(),
            ..get_valid_request()
        };
        assert_eq!(invalid_request.is_valid(), false)
    }
    #[test]
    fn given_request_with_invalid_login_is_valid_returns_false() {
        static INVALID_LOGIN: &str = "op";
        let invalid_request = RegisterRequest {
            login: INVALID_LOGIN.into(),
            ..get_valid_request()
        };
        assert_eq!(invalid_request.is_valid(), false)
    }

    #[test]
    fn given_request_with_spaces_trim_inputs_removes_spaces() {
        static PASSWORD_WITH_SPACES: &str = "  aw4vrtaa[wi93@sfe ";
        static TRIMMED_PASSWORD: &str = "aw4vrtaa[wi93@sfe";
        static FIRST_NAME_WITH_SPACES: &str = "  Edd ";
        static TRIMMED_FIRST_NAME: &str = "Edd";
        static LOGIN_WITH_SPACES: &str = "  edd11001     ";
        static TRIMMED_LOGIN: &str = "edd11001";
        let mut request = RegisterRequest {
            first_name: FIRST_NAME_WITH_SPACES.into(),
            login: LOGIN_WITH_SPACES.into(),
            password: PASSWORD_WITH_SPACES.into(),
        };

        request.trim_inputs();

        assert_eq!(request.first_name, TRIMMED_FIRST_NAME);
        assert_eq!(request.login, TRIMMED_LOGIN);
        assert_eq!(request.password, TRIMMED_PASSWORD);

        assert!(request.is_valid());
    }
}
