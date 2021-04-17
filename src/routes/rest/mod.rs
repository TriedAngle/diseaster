use actix_web::web::ServiceConfig;

mod crafting;
mod emoji;
mod food;
mod game;
mod helper;
mod operation;
mod player;
mod recipe;
mod story;

pub fn endpoints(config: &mut ServiceConfig) {
    crafting::endpoints(config);
    emoji::endpoints(config);
    food::endpoints(config);
    game::endpoints(config);
    helper::endpoints(config);
    operation::endpoints(config);
    player::endpoints(config);
    recipe::endpoints(config);
    story::endpoints(config);
}
