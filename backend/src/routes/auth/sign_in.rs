use rocket::response::Redirect;

use super::lib::{get_fail_redirect, FailReason, ProviderData};

#[get("/<provider_type>")]
pub fn handler(provider_type: &str) -> Redirect {
    if provider_type != "discord" && provider_type != "google" {
        return get_fail_redirect(&FailReason::UnknownProvider);
    }

    let provider_urls = ProviderData::new(provider_type);
    Redirect::temporary(provider_urls.auth_url)
}
