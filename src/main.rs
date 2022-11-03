use actix_web::{web, App, HttpServer};
use std::env;
use std::sync::Mutex;

mod account_endpoints;
mod board;
mod info_endpoints;
mod user;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let redis_client = redis::Client::open(
        env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379/1".to_string()),
    )
    .expect("Could not find \"REDIS_URL\" environment variable");
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
        App::new()
            .app_data(state.clone())
            .service(info_endpoints::get_board)
            .service(info_endpoints::get_current_user_id)
            .service(account_endpoints::create_account)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

struct AppState {
    redis: Mutex<redis::aio::Connection>,
    board: Mutex<Vec<Vec<board::Tile>>>,
    secret_key: String,
}
