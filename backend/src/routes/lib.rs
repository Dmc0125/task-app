use rocket::serde::{Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SuccessResponse<T> {
    success: bool,
    data: T,
}

impl<T> SuccessResponse<T> {
  pub fn new(data: T) -> SuccessResponse<T> {
    SuccessResponse { success: true, data }
  }
}
