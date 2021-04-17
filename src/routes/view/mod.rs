use crate::model::{Emoji, Food, Operation, Recipe};
use actix_web::web::ServiceConfig;
use actix_web::{error, get, web, Error, HttpResponse};
use sqlx::PgPool;
use tera::Tera;

pub fn endpoints(config: &mut ServiceConfig) {
    config
        .service(emojis)
        .service(operations)
        .service(foods)
        .service(recipes);
}

#[get("/emojis")]
async fn emojis(tmpl: web::Data<Tera>, pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let items = Emoji::all(&pool).await.unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("title", "Emoji");
    ctx.insert("emojis", &items);

    let body = tmpl
        .render("views/emoji.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/operations")]
async fn operations(tmpl: web::Data<Tera>, pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let items = Operation::all(&pool).await.unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("title", "Operation");
    ctx.insert("operations", &items);

    let body = tmpl
        .render("views/operation.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/foods")]
async fn foods(tmpl: web::Data<Tera>, pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let items = Food::all(&pool).await.unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("title", "Food");
    ctx.insert("foods", &items);

    let body = tmpl
        .render("views/food.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/recipes")]
async fn recipes(tmpl: web::Data<Tera>, pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let items = Recipe::all(&pool).await.unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("title", "Recipe");
    ctx.insert("recipes", &items);

    let body = tmpl
        .render("views/recipe.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
