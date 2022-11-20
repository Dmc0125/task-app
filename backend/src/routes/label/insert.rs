use rocket::{
    http::Status,
    serde::{json::Json, Deserialize, Serialize},
};

use backend::{entities::label, establish_db_connection};
use sea_orm::{ActiveModelTrait, DbErr};

use crate::routes::lib::{validate_len, AuthenticatedUser, ErrorResponse, SuccessResponse};

use super::lib::validate_label_color;

#[post("/label", data = "<data>")]
pub async fn handler(
    data: Json<NewLabel>,
    user: AuthenticatedUser,
) -> Result<Json<SuccessResponse<SavedLabel>>, ErrorResponse> {
    let server_err_response = ErrorResponse::new(None, Status::InternalServerError);
    let db_res = establish_db_connection().await;
    if let Err(_) = db_res {
        return Err(server_err_response);
    }
    let db = db_res.unwrap();

    if let Some(desc) = &data.description {
        let desc_len_err = validate_len(desc, 1, 30, "Description");
        if let Some(err) = desc_len_err {
            return Err(err);
        }
    }

    let clr_lowercase = &data.color.to_lowercase();
    let clr_err = validate_label_color(clr_lowercase);
    if let Some(err) = clr_err {
        return Err(err);
    }

    let insert_res = label::ActiveModel {
        user_id: sea_orm::ActiveValue::Set(user.user_id),
        workspace_id: sea_orm::ActiveValue::Set(data.workspace_id),
        color: sea_orm::ActiveValue::Set(data.color.clone()),
        description: sea_orm::ActiveValue::Set(data.description.clone()),
        ..Default::default()
    }
    .save(&db)
    .await;

    match insert_res {
        Ok(label_model) => Ok(Json(SuccessResponse::new(SavedLabel {
            workspace_id: data.workspace_id,
            color: clr_lowercase.clone(),
            description: data.description.clone(),
            id: label_model.id.unwrap(),
        }))),
        Err(db_err) => match db_err {
            DbErr::Query(query_err) => {
                if query_err.to_string().contains("fk_label_id_workspace_id") {
                    return Err(ErrorResponse::new(
                            Some(format!("Could not create label with workspace id {}, workspace does not exist", data.workspace_id)),
                            Status::BadRequest,
                        ));
                }
                Err(server_err_response)
            }
            _ => Err(server_err_response),
        },
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewLabel {
    pub workspace_id: i32,
    pub color: String,
    pub description: Option<String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SavedLabel {
    pub workspace_id: i32,
    pub color: String,
    pub description: Option<String>,
    pub id: i32,
}
