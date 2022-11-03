use crate::board;
use crate::AppState;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder, Result};
use serde_json::json;

#[get("/board")]
async fn get_board(state: web::Data<AppState>) -> impl Responder {
    let board: Vec<Vec<board::Tile>> = state.board.lock().unwrap().to_vec();

    HttpResponse::Ok().json(board)
}

#[get("/@me/id")]
async fn get_current_user_id(request: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    let user_id = match require_authentication(request, state) {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    HttpResponse::Ok().json(json!({ "id": user_id }))
}

fn require_authentication(
    request: HttpRequest,
    state: web::Data<AppState>,
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
