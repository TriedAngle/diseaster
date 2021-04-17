use actix_web::web::ServiceConfig;

mod index;
mod rest;
mod view;

pub fn endpoints(config: &mut ServiceConfig) {
    index::endpoints(config);
    rest::endpoints(config);
    view::endpoints(config);
}
