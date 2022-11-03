use actix_web::{post, Responder, web, HttpResponse};
use crate::{AppState, user::User};
use serde_derive::Deserialize;
use redis::AsyncCommands;

#[derive(Deserialize)]
struct AccountCreationQuery {
    username: String,
}

#[post("/@me")]
async fn create_account(query: web::Query<AccountCreationQuery>, state: web::Data<AppState>) -> impl Responder {
    let id: u32 = 0;
    let position = (0,0);
    
    let user = User {
        id,
        username: query.username.clone(),
        action_points: 0,
        position
    };
    
    let encoded_user = serde_json::to_string(&user).unwrap();
    
    let redis_client = state.redis.lock().unwrap();
    redis_client.set(format!("users.{id}"), encoded_user);

    HttpResponse::Ok()
}
