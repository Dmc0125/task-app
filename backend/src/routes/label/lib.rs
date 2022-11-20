use lazy_static::lazy_static;
use regex::Regex;
use rocket::http::Status;

use crate::routes::lib::ErrorResponse;

pub fn validate_label_color(clr: &String) -> Option<ErrorResponse> {
    lazy_static! {
        static ref CLR_REGEX: Regex = Regex::new(r"^(#([0-9a-f]{3}){1,2})$").unwrap();
    }

    match CLR_REGEX.is_match(clr) {
        true => None,
        false => Some(ErrorResponse::new(
            Some(format!("Color {} is not a valid hex color (#123abc)", clr)),
            Status::UnprocessableEntity,
        )),
    }
}
