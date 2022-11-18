use hex;
use hmac_sha256::HMAC;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    response,
    serde::{json::Json, Serialize},
    Request, Response,
};
use serde_json::json as serde_json;
use std::io::Cursor;

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

#[derive(Debug)]
pub struct ErrorResponse {
    body: ErrorResponseBody,
    status: Status,
}

impl ErrorResponse {
    pub fn new<T: Into<String>>(err_message: T, status: Status) -> ErrorResponse {
        ErrorResponse {
            body: ErrorResponseBody {
                success: false,
                error: err_message.into(),
            },
            status,
        }
    }
}

impl<'r> response::Responder<'r, 'static> for ErrorResponse {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let res_body = serde_json!(self.body).to_string();
        let build = &mut Response::build();
        build
            .status(self.status)
            .sized_body(res_body.len(), Cursor::new(res_body))
            .ok()
    }
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponseBody {
    success: bool,
    error: String,
}

impl ErrorResponseBody {
    pub fn new<T: Into<String>>(error_message: T) -> ErrorResponseBody {
        ErrorResponseBody {
            success: false,
            error: error_message.into(),
        }
    }
}

pub struct AuthenticatedUser {
    pub user_id: i32,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = Json<ErrorResponseBody>;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = request.cookies();
        let fail_outcome = Outcome::Failure((
            Status::Unauthorized,
            Json(ErrorResponseBody::new("Not authenticated")),
        ));

        match cookies.get("id") {
            Some(cookie) => {
                let parsed_cookie = cookie.value();
                let id_with_signature = parsed_cookie.split(".").collect::<Vec<&str>>();

                if id_with_signature.len() == 2 {
                    let id = id_with_signature[0];
                    let signature = id_with_signature[1];

                    let is_authenticated = verify_signature(id.into(), signature.into());

                    if is_authenticated {
                        return Outcome::Success(AuthenticatedUser {
                            user_id: id.parse::<i32>().unwrap(),
                        });
                    }
                }

                fail_outcome
            }
            None => fail_outcome,
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
