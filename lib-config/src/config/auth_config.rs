#[derive(serde::Deserialize)]
pub struct AuthConfig {
    pub server: String,
    pub user: UserCredentials,
}

#[derive(serde::Deserialize)]
pub struct UserCredentials {
    pub username: Option<String>,
    pub password: Option<String>,
}
