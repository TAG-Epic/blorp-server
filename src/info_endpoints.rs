use actix_web::{get, Responder, web, HttpRequest, HttpResponse};
use crate::AppState;
use crate::board;
use crate::require_authentication;


#[get("/board")]
async fn get_board(request: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    let user_id = require_authentication!(request, state);
    let board: Vec<Vec<board::Tile>> = state.board.lock().unwrap().to_vec();


    HttpResponse::Ok().json(board)
}
