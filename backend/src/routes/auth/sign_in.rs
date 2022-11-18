use rocket::response::Redirect;

use super::lib::get_provider_data;
use backend::get_env_var;
use urlencoding::encode;

#[get("/<provider_type>")]
pub fn handler(provider_type: &str) -> Redirect {
    let client_fail_url = get_env_var("CLIENT_SIGNIN_FAIL_URL");

    if provider_type != "discord" && provider_type != "google" {
        return Redirect::to(format!(
            "{}?error_msg={}",
            client_fail_url,
            encode("Unknown provider type")
        ));
    }

    let provider_urls = get_provider_data(provider_type);
    Redirect::to(provider_urls.auth_url)
}
