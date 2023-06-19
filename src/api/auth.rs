use lemmy_api_common::{person::Login, sensitive::Sensitive};

pub fn login(username_or_email: String, password: String) -> std::result::Result<lemmy_api_common::person::LoginResponse, reqwest::Error> {
    let params = Login {
        username_or_email: Sensitive::new(username_or_email),
        password: Sensitive::new(password)
    };

    super::post("/user/login", &params)
}
