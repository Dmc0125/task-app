use reqwest::{Client, Error, Response as ReqResponse};
use rocket::{response::Redirect, serde::Deserialize};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, QueryFilter, TransactionTrait,
};
use std::collections::HashMap;
use urlencoding::encode;

use crate::routes::lib::create_signature;

use super::lib::{get_fail_redirect, get_provider_data, RedirectWithCookie};
use backend::{
    entities::{
        prelude::{SocialProfile, User},
        sea_orm_active_enums::SocialProviderType,
        social_profile, user,
    },
    establish_db_connection, get_env_var,
};

#[get("/<provider_type>?<code>")]
pub async fn success_handler(
    provider_type: &str,
    code: &str,
) -> Result<RedirectWithCookie, Redirect> {
    if provider_type != "discord" && provider_type != "google" {
        return Err(get_fail_redirect());
    }

    let provider_data = get_provider_data(provider_type);
    let req_client = Client::new();

    let mut token_form_body = HashMap::new();
    token_form_body.insert("grant_type", "authorization_code");
    token_form_body.insert("redirect_uri", &provider_data.redirect_url);
    token_form_body.insert("client_id", &provider_data.client_id);
    token_form_body.insert("client_secret", &provider_data.client_secret);
    token_form_body.insert("code", code);

    let token_res = req_client
        .post(&provider_data.token_url)
        .form(&token_form_body)
        .send()
        .await;
    let token_body = handle_res_body::<TokenResponse>(token_res).await.unwrap();

    let profile_res = req_client
        .get(&provider_data.profile_url)
        .header(
            "authorization",
            format!("Bearer {}", token_body.access_token),
        )
        .send()
        .await;
    let profile_body = match provider_data.provider {
        SocialProviderType::Discord => {
            let body = handle_res_body::<DiscordProfileResponse>(profile_res)
                .await
                .unwrap();
            UserSocialProfileData {
                provider_id: body.id,
                provider_username: body.username,
            }
        }
        SocialProviderType::Google => {
            let body = handle_res_body::<GoogleProfileResponse>(profile_res)
                .await
                .unwrap();
            UserSocialProfileData {
                provider_id: body.sub,
                provider_username: String::from(""),
            }
        }
    };

    let db = unwrap_result_or_redirect(establish_db_connection().await).unwrap();
    let existing_social_profile = unwrap_result_or_redirect(
        SocialProfile::find()
            .filter(social_profile::Column::ProviderId.eq(profile_body.provider_id.clone()))
            .one(&db)
            .await,
    )
    .unwrap();

    match existing_social_profile {
        Some(sp) => {
            let existing_user =
                unwrap_result_or_redirect(User::find_by_id(sp.user_id).one(&db).await).unwrap();

            if existing_user == None {
                return Err(get_fail_redirect());
            }

            let existing_user_id = existing_user.unwrap().id.to_string();
            let signature = create_signature(&existing_user_id);

            Ok(RedirectWithCookie::new(format!(
                "id={}.{}; HttpOnly=true; Max-Age=86400; Path=/; SameSite=Strict; Secure=true;",
                existing_user_id, signature
            )))
        }
        None => {
            let user_id = unwrap_result_or_redirect(
                db.transaction::<_, String, DbErr>(|tx| {
                    Box::pin(async move {
                        let saved_user = user::ActiveModel {
                            default_social_profile: ActiveValue::Set(
                                provider_data.provider.clone(),
                            ),
                            ..Default::default()
                        }
                        .save(tx)
                        .await?;

                        let saved_user_id = saved_user.id.unwrap();
                        social_profile::ActiveModel {
                            user_id: ActiveValue::Set(saved_user_id),
                            provider_id: ActiveValue::Set(profile_body.provider_id.clone()),
                            provider_username: ActiveValue::Set(
                                profile_body.provider_username.clone(),
                            ),
                            provider_type: ActiveValue::Set(provider_data.provider),
                            ..Default::default()
                        }
                        .save(tx)
                        .await?;

                        Ok(saved_user_id.to_string())
                    })
                })
                .await,
            )
            .unwrap();
            let signature = create_signature(&user_id);

            Ok(RedirectWithCookie::new(format!(
                "id={}.{}; HttpOnly=true; Max-Age=86400; Path=/; SameSite=Strict; Secure=true;",
                user_id, signature,
            )))
        }
    }
}

#[get("/<_provider_type>?<_error>&<_error_description>", rank = 2)]
pub fn error_handler(_provider_type: &str, _error: &str, _error_description: &str) -> Redirect {
    let client_url = get_env_var("CLIENT_URL");
    Redirect::to(client_url)
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct TokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct GoogleProfileResponse {
    sub: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct DiscordProfileResponse {
    id: String,
    username: String,
}

struct UserSocialProfileData {
    provider_id: String,
    provider_username: String,
}

pub async fn handle_res_body<T>(res: Result<ReqResponse, Error>) -> Result<T, Redirect>
where
    T: for<'a> Deserialize<'a>,
{
    match res {
        Ok(r) => {
            let parsed = r.json::<T>().await;
            match parsed {
                Ok(p) => Ok(p),
                Err(_) => Err(get_fail_redirect()),
            }
        }
        Err(_) => Err(get_fail_redirect()),
    }
}

fn unwrap_result_or_redirect<T, E>(res: Result<T, E>) -> Result<T, Redirect> {
    let client_fail_url = get_env_var("CLIENT_SIGNIN_FAIL_URL");
    match res {
        Ok(success) => Ok(success),
        Err(_) => Err(Redirect::to(format!(
            "{}?error_msg={}",
            client_fail_url,
            encode("Server error")
        ))),
    }
}
