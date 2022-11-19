use rocket::http::Status;

use crate::routes::lib::ErrorResponse;

pub fn validate_len(
    input: &String,
    min_len: usize,
    max_len: usize,
    property: &str,
) -> Option<ErrorResponse> {
    if input.len() < min_len || input.len() > max_len {
        return Some(ErrorResponse::new(
            Some(format!(
                "{} length must be between {} and {} characters",
                property, min_len, max_len,
            )),
            Status::BadRequest,
        ));
    }

    None
}
