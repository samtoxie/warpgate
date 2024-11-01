use openidconnect::core::CoreIdToken;

#[derive(Clone, Debug)]
pub struct SsoLoginResponse {
    pub preferred_username: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub groups: Option<Vec<String>>,
    pub id_token: CoreIdToken,
}
