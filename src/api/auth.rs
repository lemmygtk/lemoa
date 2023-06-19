use lemmy_api_common::{person::{LoginResponse, Login}, sensitive::Sensitive};

pub fn login(username_or_email: String, password: String) -> std::result::Result<LoginResponse, reqwest::Error> {
    let params = Login {
        username_or_email: Sensitive::new(username_or_email),
        password: Sensitive::new(password)
    };

    super::get("/user/login", &params)
}
