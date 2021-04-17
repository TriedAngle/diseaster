use actix_web::web::ServiceConfig;

mod chance;
mod department;
mod disease;
mod doctor;
mod symptom;

pub fn endpoints(config: &mut ServiceConfig) {
    symptom::endpoints(config);
    chance::endpoints(config);
    disease::endpoints(config);
    department::endpoints(config);
    doctor::endpoints(config);
}
