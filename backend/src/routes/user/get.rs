use crate::routes::lib::{AuthenticatedUser, ErrorResponse, SuccessResponse};

use rocket::{
    http::Status,
    serde::{json::Json, Serialize},
};
use sea_orm::EntityTrait;

use backend::{
    entities::prelude::{SocialProfile, User},
    establish_db_connection,
};

#[get("/user")]
pub async fn handler(
    authenticated_user: AuthenticatedUser,
) -> Result<Json<SuccessResponse<FoundUserData>>, ErrorResponse> {
    let db_res = establish_db_connection().await;
    if let Err(_) = db_res {
        return Err(ErrorResponse::new(None, Status::InternalServerError));
    }
    let db = db_res.unwrap();
    let saved_user_res = User::find_by_id(authenticated_user.user_id)
        .find_with_related(SocialProfile)
        .all(&db)
        .await;

    if let Err(_) = saved_user_res {
        return Err(ErrorResponse::new(None, Status::InternalServerError));
    }

    let saved_user_vec = saved_user_res.unwrap();
    if saved_user_vec.len() == 0 {
        // User has to exit if request contained signed cookie
        // Respond with server error
        return Err(ErrorResponse::new(None, Status::InternalServerError));
    }

    let (saved_user, social_profiles) = &saved_user_vec[0];
    let default_social_profile_option = social_profiles
        .iter()
        .find(|&sp| sp.provider_type.eq(&saved_user.default_social_profile));

    if None == default_social_profile_option {
        // Default social profile has to exist if user exists
        // Respond with server error
        return Err(ErrorResponse::new(None, Status::InternalServerError));
    }

    let default_social_profile = default_social_profile_option.unwrap();

    Ok(Json(SuccessResponse::new(FoundUserData {
        provider_type: saved_user.default_social_profile.to_string().replace("'", ""),
        username: default_social_profile.provider_username.clone(),
        avatar: default_social_profile.provider_avatar.clone(),
    })))
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FoundUserData {
    pub provider_type: String,
    pub username: String,
    pub avatar: Option<String>,
}
