use actix_web::{web, App, HttpServer};
use std::env;
use std::sync::Mutex;
use actix_cors::Cors;

mod account_endpoints;
mod board;
mod info_endpoints;
mod user;
mod utils;
mod board_endpoints;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let redis_client = redis::Client::open(
        env::var("REDIS_URI").unwrap_or_else(|_| "redis://localhost:6379".to_string()),
    ).unwrap();
    let redis_connection = redis_client.get_async_connection().await.unwrap();

    let secret_key = env::var("SECRET").expect("Could not find \"SECRET\" environment variable");

    println!("Connected to redis!");

    let app_state = AppState {
        redis: Mutex::new(redis_connection),
        board: Mutex::new(board::create_board()),
        secret_key,
    };
    let state = web::Data::new(app_state);


    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(state.clone())
            .service(info_endpoints::get_board)
            .service(info_endpoints::get_current_user)
            .service(info_endpoints::get_all_users)
            .service(info_endpoints::get_user)
            .service(account_endpoints::create_account)
            .service(board_endpoints::dig_at)
            .service(board_endpoints::move_player)
            .service(board_endpoints::attack_player)
            .service(board_endpoints::gift_player)
            .service(account_endpoints::test_create_points)
            .service(account_endpoints::upgrade_range)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

pub struct AppState {
    redis: Mutex<redis::aio::Connection>,
    board: Mutex<Vec<Vec<board::Tile>>>,
    secret_key: String,
}
