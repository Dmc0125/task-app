use rocket::{
    http::Status,
    serde::{json::Json, Deserialize, Serialize},
};

use backend::{
    entities::{label, prelude::Label},
    establish_db_connection,
};
use sea_orm::{ActiveModelTrait, EntityTrait};

use crate::routes::lib::{validate_len, AuthenticatedUser, ErrorResponse, SuccessResponse};

use super::lib::validate_label_color;

#[patch("/label/<label_id>", data = "<data>")]
pub async fn handler(
    label_id: i32,
    data: Json<LabelDataToUpdate>,
    user: AuthenticatedUser,
) -> Result<Json<SuccessResponse<UpdatedLabel>>, ErrorResponse> {
    if None == data.color && None == data.description {
        return Err(ErrorResponse::new(
            Some("Either color or description has to be provided".into()),
            Status::BadRequest,
        ));
    }

    let server_err_response = ErrorResponse::new(None, Status::InternalServerError);
    let db_res = establish_db_connection().await;
    if let Err(_) = db_res {
        return Err(server_err_response);
    }
    let db = db_res.unwrap();

    let old_label_select_res = Label::find_by_id(label_id).one(&db).await;
    if let Err(_) = old_label_select_res {
        return Err(server_err_response);
    }

    let old_label = old_label_select_res.unwrap();
    let not_found_err_response = ErrorResponse::new(
        Some(format!("Label with id {} does not exist", label_id)),
        Status::NotFound,
    );

    match old_label {
        None => Err(not_found_err_response),
        Some(old_label_model) => {
            if old_label_model.user_id != user.user_id {
                return Err(not_found_err_response);
            }

            let mut label_to_update: label::ActiveModel = old_label_model.into();

            if let Some(clr) = &data.color {
                let clr_lowercase = clr.to_lowercase();
                let clr_err = validate_label_color(&clr_lowercase);
                if let Some(err) = clr_err {
                    return Err(err);
                }

                label_to_update.color = sea_orm::ActiveValue::Set(clr_lowercase);
            }

            if let Some(desc) = &data.description {
                let desc_len_err = validate_len(desc, 1, 30, "Description");
                if let Some(err) = desc_len_err {
                    return Err(err);
                }

                label_to_update.description = sea_orm::ActiveValue::Set(Some(desc.to_string()));
            }

            let updated_label_res = label_to_update.update(&db).await;
            match updated_label_res {
                Err(_) => Err(server_err_response),
                Ok(updated_label_model) => Ok(Json(SuccessResponse::new(UpdatedLabel {
                    workspace_id: updated_label_model.workspace_id,
                    description: updated_label_model.description,
                    color: updated_label_model.color,
                }))),
            }
        }
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LabelDataToUpdate {
    pub description: Option<String>,
    pub color: Option<String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdatedLabel {
    pub workspace_id: i32,
    pub description: Option<String>,
    pub color: String,
}
