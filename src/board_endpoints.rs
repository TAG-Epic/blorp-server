use crate::{board, AppState, user, utils};
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use redis::AsyncCommands;

#[post("/board/dig")]
async fn dig_at(request: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    let user_id = match utils::require_authentication(&request, &state) {
        Ok(id) => id,
        Err(resp) => return resp,
    };
    utils::process_award_points(&user_id, &state).await;

    let mut redis_client = state.redis.lock().unwrap();
    let encoded_user = redis_client.get::<String, String>(format!("users.{user_id}")).await.unwrap();

    let mut user: user::User = serde_json::from_str(&encoded_user).unwrap();
    let x: usize = user.position.0.into();
    let y: usize = user.position.1.into();
    let current_tile = &state.board.lock().unwrap()[x][y];
    
    match current_tile.tile_type {
        board::TileType::RESOURCEFUL(resources) => {
            user.action_points += resources;
            let mut board = state.board.lock().unwrap();
            board[x][y].tile_type = board::TileType::EMPTY;

            return HttpResponse::Ok().json(json!({}));
        },
        _ => {
            return HttpResponse::BadRequest().json(json!({"error": "Cannot dig here"}))
        }
    }
}

#[post("/board/move/{direction}")]
async fn move_player(request: HttpRequest, path: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let user_id = match utils::require_authentication(&request, &state) {
        Ok(id) => id,
        Err(resp) => return resp,
    };
    utils::process_award_points(&user_id, &state).await;


    let mut redis_client = state.redis.lock().unwrap();
    let encoded_user = redis_client.get::<String, String>(format!("users.{user_id}")).await.unwrap();
    let mut user: user::User = serde_json::from_str(&encoded_user).unwrap();

    let new_position = match path.into_inner().as_str() {
        "UP" => {
            (user.position.0, user.position.1.wrapping_add(1))
        },
        "DOWN" => {
            (user.position.0, user.position.1.wrapping_sub(1))
        },
        "LEFT" => {
            (user.position.0.wrapping_sub(1), user.position.1)
        },
        "RIGHT" => {
            (user.position.0.wrapping_add(1), user.position.1)
        },
        _ => {
            (0, 0)
        }
    };

    let new_position = (new_position.0.min(100).max(0), new_position.1.min(100).max(0));

    user.position = new_position;
    
    if let Some(points) = user.action_points.checked_sub(1) {
        user.action_points = points;
    } else {
        return HttpResponse::BadRequest().json(json!({"error": "Not enough action points"}));
    }

    let encoded_user = serde_json::to_string(&user).unwrap();
    redis_client.set::<String, String, ()>(format!("users.{user_id}"), encoded_user).await.expect("Could not write user");

    HttpResponse::Ok().json(json!({}))
}

#[post("/user/{user_id}/attack")]
async fn attack_player(request: HttpRequest, path: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let user_id = match utils::require_authentication(&request, &state) {
        Ok(id) => id,
        Err(resp) => return resp,
    };
    utils::process_award_points(&user_id, &state).await;


    let mut redis_client = state.redis.lock().unwrap();

    let encoded_user = redis_client.get::<String, String>(format!("users.{user_id}")).await.unwrap();
    let mut user: user::User = serde_json::from_str(&encoded_user).unwrap();

    if user.health == 0 {
        return HttpResponse::BadRequest().json(json!({"error": "Imagine being dead"}));
    }

    let other_user_id = path.into_inner();
    let other_user = redis_client.get::<String, String>(format!("users.{other_user_id}")).await;
    if other_user.is_err() {
        return HttpResponse::BadRequest().json(json!({"error": "Could not find user"}));
    }
    let mut other_user: user::User = serde_json::from_str(&other_user.unwrap()).unwrap();

    if other_user.health == 0 {
        return HttpResponse::BadRequest().json(json!({"error": "Cannot attack dead players"}));
    }

    let distance = ((user.position.0 as i8 - other_user.position.0 as i8).abs() + (user.position.1 as i8 - other_user.position.1 as i8).abs()) as u8;

    if distance > user.range {
        return HttpResponse::BadRequest().json(json!({"error": "Not enough range"}));
    }

    if let Some(points) = user.action_points.checked_sub(1) {
        user.action_points = points;
    } else {
        return HttpResponse::BadRequest().json(json!({"error": "Not enough points"}));
    }
    other_user.health = other_user.health - 1;

    // Save
    let encoded_user = serde_json::to_string(&user).unwrap();
    let encoded_other_user = serde_json::to_string(&other_user).unwrap();

    redis_client.set::<String, String, ()>(format!("users.{user_id}"), encoded_user).await.expect("Could not write user");
    redis_client.set::<String, String, ()>(format!("users.{other_user_id}"), encoded_other_user).await.expect("Could not write user");

    HttpResponse::Ok().json(json!({}))
}

#[post("/user/{user_id}/gift")]
async fn gift_player(request: HttpRequest, path: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let user_id = match utils::require_authentication(&request, &state) {
        Ok(id) => id,
        Err(resp) => return resp,
    };
    utils::process_award_points(&user_id, &state).await;


    let mut redis_client = state.redis.lock().unwrap();

    let encoded_user = redis_client.get::<String, String>(format!("users.{user_id}")).await.unwrap();
    let mut user: user::User = serde_json::from_str(&encoded_user).unwrap();

    let other_user_id = path.into_inner();
    let other_user = redis_client.get::<String, String>(format!("users.{other_user_id}")).await;
    if other_user.is_err() {
        return HttpResponse::BadRequest().json(json!({"error": "Could not find user"}));
    }
    let mut other_user: user::User = serde_json::from_str(&other_user.unwrap()).unwrap();

    if user.action_points == 0 {
        return HttpResponse::Ok().json(json!({"error": "Not enough action points"}));
    }

    user.action_points = user.action_points - 1;
    other_user.action_points = other_user.action_points + 1;

    // Save
    let encoded_user = serde_json::to_string(&user).unwrap();
    let encoded_other_user = serde_json::to_string(&other_user).unwrap();

    redis_client.set::<String, String, ()>(format!("users.{user_id}"), encoded_user).await.expect("Could not write user");
    redis_client.set::<String, String, ()>(format!("users.{other_user_id}"), encoded_other_user).await.expect("Could not write user");

    HttpResponse::Ok().json(json!({}))
}
