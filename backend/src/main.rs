#[macro_use]
extern crate rocket;

use dotenvy::dotenv;
use rocket::serde::json::Json;
use routes::lib::SuccessResponse;

mod routes;

#[get("/")]
fn index() -> Json<SuccessResponse<&'static str>> {
    Json(routes::lib::SuccessResponse::new("This is an API"))
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .mount("/", routes![index])
        .mount(
            "/api/v1/auth/signin",
            routes![routes::auth::sign_in::handler],
        )
        .mount(
            "/api/v1/auth/callback",
            routes![
                routes::auth::callback::success_handler,
                routes::auth::callback::error_handler
            ],
        )
        .mount(
            "/api/v1/auth/signout",
            routes![routes::auth::sign_out::handler],
        )
        .mount(
            "/api/v1",
            routes![
                routes::workspace::insert::handler,
                routes::user::get::handler,
            ],
        )
}
