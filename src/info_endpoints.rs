use crate::{board, AppState, user, utils};
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use redis::AsyncCommands;

#[get("/board")]
async fn get_board(state: web::Data<AppState>) -> impl Responder {
    let board: Vec<Vec<board::Tile>> = state.board.lock().unwrap().to_vec();

    HttpResponse::Ok().json(board)
}

#[get("/@me")]
async fn get_current_user(request: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    let user_id = match utils::require_authentication(&request, &state) {
        Ok(id) => id,
        Err(resp) => return resp,
    };
    utils::process_award_points(&user_id, &state).await;

    let mut redis_client = state.redis.lock().unwrap();
    let encoded_user = redis_client.get::<String, String>(format!("users.{user_id}")).await.unwrap();

    let user: user::User = serde_json::from_str(&encoded_user).unwrap();

    HttpResponse::Ok().json(json!(user))
}

