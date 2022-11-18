use rocket::response::Redirect;

use super::lib::{get_fail_redirect, get_provider_data, FailReason};

#[get("/<provider_type>")]
pub fn handler(provider_type: &str) -> Redirect {
    if provider_type != "discord" && provider_type != "google" {
        return get_fail_redirect(&FailReason::UnknownProvider);
    }

    let provider_urls = get_provider_data(provider_type);
    Redirect::to(provider_urls.auth_url)
}
