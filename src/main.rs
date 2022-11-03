use actix_web::{web, App, HttpServer};
use std::env;
use std::sync::Mutex;

mod account_endpoints;
mod board;
mod info_endpoints;
mod user;
//#[macro_use] mod macros;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let redis_client = redis::Client::open(env::var("REDIS_URI").unwrap()).unwrap();
    let redis_connection = redis_client.get_async_connection().await.unwrap();

    let secret_key = env::var("SECRET").unwrap();

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

// TODO: Move to macros.rs
//
#[macro_export]
macro_rules! require_authentication {
    ($request:expr, $state:expr) => {{
        if let Some(authentication) = $request.headers().get("authentication") {
            use itsdangerous::{default_builder, Signer};
            let signer = default_builder($state.secret_key.clone()).build();

            let unsigned = signer.unsign(&authentication.to_str().unwrap());

            if unsigned.is_err() {
                return actix_web::HttpResponse::Unauthorized().body("Invalid auth");
            }
            unsigned.unwrap().to_string()
        } else {
            return actix_web::HttpResponse::Unauthorized().body("No authentication provided");
        }
    }};
}
