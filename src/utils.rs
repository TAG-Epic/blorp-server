use actix_web::{web, HttpRequest, HttpResponse};
use crate::{AppState, user};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use redis::AsyncCommands;
use serde_json::json;

pub async fn process_award_points(user_id: &str, state: &web::Data<AppState>) {
    let mut redis_client = state.redis.lock().unwrap();
    let encoded_user = redis_client.get::<String, String>(format!("users.{user_id}")).await.unwrap();
    let mut user: user::User = serde_json::from_str(&encoded_user).unwrap();

    let hours_since_creation_date: u32 = (SystemTime::now().duration_since(UNIX_EPOCH+Duration::from_secs(user.creation_date.into())).unwrap().as_secs() / 60 / 60).try_into().unwrap();
    let awarded_points = hours_since_creation_date - user.awarded_points;

    user.awarded_points += awarded_points;
    user.action_points += awarded_points;

    println!("Awarded {awarded_points} to {user_id}");

    let encoded_user = json!(user).to_string();

    redis_client.set::<String, String, ()>(format!("users.{user_id}"), encoded_user).await.expect("Could not create user");
}

pub fn require_authentication(
    request: &HttpRequest,
    state: &web::Data<AppState>,
) -> Result<String, HttpResponse> {
    if let Some(authentication) = request.headers().get("authentication") {
        use itsdangerous::{default_builder, Signer};
        let signer = default_builder(state.secret_key.clone()).build();

        let unsigned = signer.unsign(&authentication.to_str().unwrap());
        if unsigned.is_err() {
            Err(HttpResponse::Unauthorized().body("Invalid auth"))
        } else {
            Ok(unsigned.unwrap().to_string())
        }
    } else {
        Err(HttpResponse::Unauthorized().body("No authentication provided"))
    }
}
