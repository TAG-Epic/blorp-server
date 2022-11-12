use crate::{board, AppState, user, utils};
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use redis::AsyncCommands;

#[get("/board")]
async fn get_board(state: web::Data<AppState>) -> impl Responder {
    let board: Vec<Vec<board::Tile>> = state.board.lock().await.to_vec();

    HttpResponse::Ok().json(board)
}
#[get("/user/{user_id}")]
async fn get_user(path: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let user_id = path.into_inner();
    println!("GET /user/{user_id}");
    let mut redis_client = state.redis.lock().await;
    let user = redis_client.get::<String, String>(format!("users.{user_id}")).await;
    
    if user.is_err() {
        return HttpResponse::BadRequest().json(json!({"error": "Invalid user id"}));
    }
    let encoded_user = user.unwrap();
    let user: user::User = serde_json::from_str(&encoded_user).unwrap();

    HttpResponse::Ok().json(json!(user))
}

#[get("/user/@me")]
async fn get_current_user(request: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    println!("GET /user/@me");
    let user_id = match utils::require_authentication(&request, &state) {
        Ok(id) => id,
        Err(resp) => return resp,
    };
    utils::process_award_points(&user_id, &state).await;

    let mut redis_client = state.redis.lock().await;
    let encoded_user = redis_client.get::<String, String>(format!("users.{user_id}")).await.unwrap();

    let user: user::User = serde_json::from_str(&encoded_user).unwrap();

    HttpResponse::Ok().json(json!(user))
}

#[get("/user/all")]
async fn get_all_users(state: web::Data<AppState>) -> impl Responder {
    let mut redis_client = state.redis.lock().await;
    let mut users = vec!();
    for key in redis_client.keys::<&str, Vec<String>>("users.*").await.unwrap() {
        let user = redis_client.get::<String, String>(key).await.unwrap();
        let user: user::User = serde_json::from_str(&user).unwrap();
        users.push(user);
    }
    HttpResponse::Ok().json(users)
}
