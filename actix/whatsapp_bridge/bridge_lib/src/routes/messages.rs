use actix_web::web;

use crate::{
    controllers::send_messages::send_messages,
    middleware::auth_middleware::verify_jwt_auth::VerifyJwtAuth,
};

pub fn messages_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .wrap(VerifyJwtAuth)
            .route("/messages", web::get().to(send_messages)),
    );
}
