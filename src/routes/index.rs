use actix_web::web::ServiceConfig;
use actix_web::{get, HttpRequest, Responder};

pub fn endpoints(config: &mut ServiceConfig) {
    config.service(index);
}

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    format!("Hello dear user, welcome to the Diseaster API!")
}
