use backend::env_err_msg;
use std::{collections::HashMap, env};

pub fn get_urls(provider: &str) -> ProviderUrls {
    if provider != "discord" || provider != "google" {
        panic!("Unknown provider {}", provider)
    }

    let base_url = env::var("BASE_URL").expect(&env_err_msg("Could not find BASE_URL"));
    let redirect_url = format!(
        "{}/auth/v1/auth/callback/{}",
        base_url,
        provider.to_lowercase()
    );

    let provider_uppercase = provider.to_uppercase();
    let client_id = env::var(format!("{}_CLIENT_ID", provider_uppercase))
        .expect(&env_err_msg("Could not find CLIENT_ID"));
    let client_secret = env::var(format!("{}_CLIENT_SECRET", provider_uppercase))
        .expect(&env_err_msg("Could not find CLIENT_SECRET"));

    match provider {
        "discord" => ProviderUrls {
            auth_url: String::from(
                "https://discord.com/api/oauth2/authorize?response_type=code&scope=identify",
            ),
            profile_url: String::from("https://discord.com/api/users/@me"),
            token_url: String::from("https://discord.com/api/oauth2/token"),
            redirect_url,
            client_id,
            client_secret,
        },
        "google" => ProviderUrls {
            auth_url: String::from(
                "https://accounts.google.com/o/oauth2/v2/auth?response_type=code&scope=openid",
            ),
            profile_url: String::from("https://openidconnect.googleapis.com/v1/userinfo"),
            token_url: String::from("https://oauth2.googleapis.com/token"),
            redirect_url,
            client_id,
            client_secret,
        },
        _ => panic!("Unknown provider {}", provider),
    }
}

pub struct ProviderUrls {
    auth_url: String,
    profile_url: String,
    token_url: String,
    redirect_url: String,
    client_id: String,
    client_secret: String,
}

impl ProviderUrls {
    pub fn get_auth_url(self: &Self) -> String {
        format!(
            "{}&client_id={}&redirect_uri={}",
            self.auth_url, self.client_id, self.redirect_url,
        )
    }

    pub fn get_token_url_and_body(self: &Self, code: &str) -> HashMap<&str, String> {
        let mut body: HashMap<&str, String> = HashMap::new();
        body.insert("grant_type", "authorization_code".into());
        body.insert("redirect_uri", String::from(&self.redirect_url));
        body.insert("client_id", String::from(&self.client_id));
        body.insert("client_secret", String::from(&self.client_secret));
        body.insert("code", code.into());

        body
    }
}
