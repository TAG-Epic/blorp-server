use crate::board;
use crate::require_authentication;
use crate::AppState;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;

#[get("/board")]
async fn get_board(state: web::Data<AppState>) -> impl Responder {
    let board: Vec<Vec<board::Tile>> = state.board.lock().unwrap().to_vec();

    HttpResponse::Ok().json(board)
}

#[get("/@me/id")]
async fn get_current_user_id(request: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    let user_id = require_authentication!(request, state);

    HttpResponse::Ok().json(json!({ "id": user_id }))
}
