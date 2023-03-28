pub(crate) struct RequestValidator {}

impl RequestValidator {
    pub fn does_password_meet_requirements(password: &str) -> bool {
        static MIN_PASSWORD_LENGTH: usize = 8;
        static MAX_PASSWORD_LENGTH: usize = 100;
        Self::is_in_length_range(password, MIN_PASSWORD_LENGTH, MAX_PASSWORD_LENGTH)
    }
    pub fn does_login_meet_requirements(login: &str) -> bool {
        static MIN_LOGIN_LENGTH: usize = 4;
        static MAX_LOGIN_LENGTH: usize = 20;
        Self::is_in_length_range(login, MIN_LOGIN_LENGTH, MAX_LOGIN_LENGTH)
    }
    pub fn does_first_name_meet_requirements(first_name: &str) -> bool {
        static MIN_FIRST_NAME_LENGTH: usize = 1;
        static MAX_FIRST_NAME_LENGTH: usize = 20;
        Self::is_in_length_range(first_name, MIN_FIRST_NAME_LENGTH, MAX_FIRST_NAME_LENGTH)
    }

    fn is_in_length_range(input: &str, min_length: usize, max_length: usize) -> bool {
        let input_length = input.chars().count();
        let allowable_range = min_length..=max_length;
        allowable_range.contains(&input_length)
    }
}
