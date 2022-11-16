#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use routes::lib::SuccessResponse;

mod routes;

#[get("/")]
fn index() -> Json<SuccessResponse<&'static str>> {
    Json(routes::lib::SuccessResponse::new("This is an API"))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api/v1/auth/", routes![routes::auth::sign_in::handler])
}
