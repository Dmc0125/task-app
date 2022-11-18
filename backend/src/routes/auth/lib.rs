use rocket::{http::Status, response, Request, Response};
use urlencoding::encode;

use backend::{entities::sea_orm_active_enums::SocialProviderType, get_env_var};

pub fn get_fail_redirect() -> response::Redirect {
    let client_fail_url = get_env_var("CLIENT_SIGNIN_FAIL_URL");
    let redirect_url = format!(
        "{}?error_msg={}",
        client_fail_url,
        encode("Unknown provider")
    );
    response::Redirect::permanent(redirect_url)
}

pub struct RedirectWithCookie {
    cookie_value: Option<String>,
}

impl RedirectWithCookie {
    pub fn new(cookie_value: String) -> RedirectWithCookie {
        RedirectWithCookie {
            cookie_value: Some(cookie_value),
        }
    }
}

impl<'r> response::Responder<'r, 'static> for RedirectWithCookie {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let client_success_url = get_env_var("CLIENT_SIGNIN_SUCCESS_URL");
        let builder_binding = &mut Response::build();
        let response_builder = builder_binding
            .status(Status::PermanentRedirect)
            .raw_header("Location", client_success_url);

        if let Some(cookie_value) = self.cookie_value {
            response_builder.raw_header("set-cookie", cookie_value);
        }

        response_builder.ok()
    }
}

pub fn get_provider_data(provider: &str) -> ProviderData {
    let base_url = get_env_var("BASE_URL");
    let redirect_url = format!(
        "{}/api/v1/auth/callback/{}",
        base_url,
        provider.to_lowercase()
    );

    let provider_uppercase = provider.to_uppercase();

    let client_id = get_env_var(format!("{}_CLIENT_ID", provider_uppercase));
    let client_secret = get_env_var(format!("{}_CLIENT_SECRET", provider_uppercase));

    match provider {
        "discord" => ProviderData {
            provider: SocialProviderType::Discord,
            auth_url: format!(
                "https://discord.com/api/oauth2/authorize?response_type=code&scope=identify&client_id={}&redirect_uri={}",
                client_id, redirect_url,
            ),
            profile_url: String::from("https://discord.com/api/users/@me"),
            token_url: String::from("https://discord.com/api/oauth2/token"),
            redirect_url,
            client_id,
            client_secret,
        },
        "google" => ProviderData {
            provider: SocialProviderType::Google,
            auth_url: format!(
                "https://accounts.google.com/o/oauth2/v2/auth?response_type=code&scope=openid&client_id={}&redirect_uri={}",
                client_id, redirect_url,
            ),
            profile_url: String::from("https://openidconnect.googleapis.com/v1/userinfo"),
            token_url: String::from("https://oauth2.googleapis.com/token"),
            redirect_url,
            client_id,
            client_secret,
        },
        _ => panic!("Unknown provider {}", provider)
    }
}

pub struct ProviderData {
    pub provider: SocialProviderType,
    pub auth_url: String,
    pub profile_url: String,
    pub token_url: String,
    pub redirect_url: String,
    pub client_id: String,
    pub client_secret: String,
}
