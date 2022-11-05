use crate::{user::User, AppState};
use actix_web::{post, web, HttpResponse, Responder};
use redis::AsyncCommands;
use serde_derive::Deserialize;
use itsdangerous::{default_builder, Signer};
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
struct AccountCreationQuery {
    username: String,
}

#[post("/@me")]
async fn create_account(
    query: web::Query<AccountCreationQuery>,
    state: web::Data<AppState>,
) -> impl Responder {
    let id: u32 = 0;
    let position = (0, 0);
    let created_at: u32 = SystemTime::now().duration_since(SystemTime::from(UNIX_EPOCH)).unwrap().as_secs().try_into().unwrap();

    let user = User {
        id,
        username: query.username.clone(),
        action_points: 0,
        position,
        awarded_points: 0,
        creation_date: created_at
    };

    let encoded_user = serde_json::to_string(&user).unwrap();

    let mut redis_client = state.redis.lock().unwrap();
    redis_client.set::<String, String, ()>(format!("users.{id}"), encoded_user).await.expect("Could not create user");

    let signer = default_builder(state.secret_key.clone()).build();
    let auth_token = signer.sign(id.to_string());


    HttpResponse::Ok().json(json!({"token": auth_token, "user": user}))
}
