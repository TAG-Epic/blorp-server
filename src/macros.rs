#[macro_export]
macro_rules! require_authentication {
    () => {
        {
            if let Some(authentication) = request.headers().get("authentication") {
                use itsdangerous::Signer;
                let signer = itsdangerous::SignerBuilder::new(state.secret_key).build();
            } else {
                return actix_web::HttpResponse::Unauthorized().body("No authentication provided");
            }
        }
    }
}
