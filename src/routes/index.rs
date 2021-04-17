use actix_web::web::ServiceConfig;
use actix_web::{get, HttpRequest, Responder};

pub fn endpoints(config: &mut ServiceConfig) {
    config.service(index);
}

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    format!("Hello dear user, welcome to the Carol Emoji Cooking Bot Api �!")
}
