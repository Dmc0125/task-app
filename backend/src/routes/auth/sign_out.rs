use super::lib::{AuthRoute, AuthSuccessRedirect};

#[get("/")]
pub fn handler() -> AuthSuccessRedirect {
    AuthSuccessRedirect {
        cookies: vec![
            (
                "set-cookie".into(),
                "id=0; HttpOnly=true; Max-Age=0; Path=/; SameSite=Strict; Secure=true;".into(),
            ),
            ("Cache-control".into(), "no-store".into()),
        ],
        route: AuthRoute::SignOut,
    }
}
