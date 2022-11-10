use crate::{user, utils, AppState};
use actix_web::{post, web, HttpResponse, Responder, HttpRequest};
use redis::AsyncCommands;
use serde_derive::Deserialize;
use itsdangerous::{default_builder, Signer};
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::{thread_rng, Rng};

#[derive(Deserialize)]
struct AccountCreationQuery {
    username: String,
}

#[post("/user/@me")]
async fn create_account(
    query: web::Query<AccountCreationQuery>,
    state: web::Data<AppState>,
) -> impl Responder {
    let id: u32 = thread_rng().gen();
    let position = (0, 0);
    let created_at: u32 = SystemTime::now().duration_since(SystemTime::from(UNIX_EPOCH)).unwrap().as_secs().try_into().unwrap();

    let user = user::User {
        id,
        username: query.username.clone(),
        action_points: 0,
        position,
        awarded_points: 0,
        creation_date: created_at,
        range: 1,
        health: 3
    };

    let encoded_user = serde_json::to_string(&user).unwrap();

    let mut redis_client = state.redis.lock().unwrap();
    redis_client.set::<String, String, ()>(format!("users.{id}"), encoded_user).await.expect("Could not create user");

    let signer = default_builder(state.secret_key.clone()).build();
    let auth_token = signer.sign(id.to_string());


    HttpResponse::Ok().json(json!({"token": auth_token, "user": user}))
}
#[post("/user/@me/points")]
async fn test_create_points(request: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    let user_id = match utils::require_authentication(&request, &state) {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    utils::process_award_points(&user_id, &state).await;


    let mut redis_client = state.redis.lock().unwrap();

    let encoded_user = redis_client.get::<String, String>(format!("users.{user_id}")).await.unwrap();
    let mut user: user::User = serde_json::from_str(&encoded_user).unwrap();

    user.action_points = 99;

    let encoded_user = serde_json::to_string(&user).unwrap();
    redis_client.set::<String, String, ()>(format!("users.{user_id}"), encoded_user).await.expect("Could not create user");

    HttpResponse::Ok().json(json!({}))
}

#[post("/user/@me/upgrade/range")]
async fn upgrade_range(request: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    let user_id = match utils::require_authentication(&request, &state) {
        Ok(id) => id,
        Err(resp) => return resp,
    };
    utils::process_award_points(&user_id, &state).await;

    let mut redis_client = state.redis.lock().unwrap();
    let encoded_user = redis_client.get::<String, String>(format!("users.{user_id}")).await.unwrap();

    let mut user: user::User = serde_json::from_str(&encoded_user).unwrap();

    if let Some(points) = user.action_points.checked_sub(1) {
        user.action_points = points;
    } else {
        return HttpResponse::BadRequest().json(json!({"error": "Not enough action points"}));
    }

    if user.range == 5 {
        return HttpResponse::BadRequest().json(json!({"error": "Max range reached"}));
    }

    if let Some(range) = user.range.checked_add(1) {
        user.range = range;
    } else {
        return HttpResponse::BadRequest().json(json!({"error": "Max range acquired"}));
    }

    let encoded_user = serde_json::to_string(&user).unwrap();
    redis_client.set::<String, String, ()>(format!("users.{user_id}"), encoded_user).await.expect("Could not create user");

    HttpResponse::Ok().json(json!({}))
}


