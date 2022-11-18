use hex;
use hmac_sha256::HMAC;
use rocket::serde::Serialize;

use backend::get_env_var;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SuccessResponse<T> {
    success: bool,
    data: T,
}

impl<T> SuccessResponse<T> {
    pub fn new(data: T) -> SuccessResponse<T> {
        SuccessResponse {
            success: true,
            data,
        }
    }
}

pub fn create_signature(user_id: &String) -> String {
    let signature_key = get_env_var("SIGNATURE_KEY");

    let mut hmac = HMAC::new(signature_key.as_bytes());
    hmac.update(user_id.as_bytes());
    let signature = hmac.finalize();

    hex::encode(signature)
}

pub fn verify_signature(user_id: String, provided_signature: String) -> bool {
    let signature_key = get_env_var("SIGNATURE_KEY");

    let mut hmac = HMAC::new(signature_key.as_bytes());
    hmac.update(user_id.as_bytes());
    let signature = hmac.finalize();

    hex::encode(signature) == provided_signature
}
