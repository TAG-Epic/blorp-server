use crate::{user::User, AppState};
use actix_web::{post, web, HttpResponse, Responder};
use redis::AsyncCommands;
use serde_derive::Deserialize;

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

    let user = User {
        id,
        username: query.username.clone(),
        action_points: 0,
        position,
    };

    let encoded_user = serde_json::to_string(&user).unwrap();

    let redis_client = state.redis.lock().unwrap();
    redis_client.set(format!("users.{id}"), encoded_user);

    HttpResponse::Ok()
}
