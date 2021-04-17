use crate::models::Chance as Model;
use actix_web::web::ServiceConfig;
use actix_web::{delete, get, patch, post, web, Error, HttpRequest, HttpResponse};
use sqlx::PgPool;

pub fn endpoints(config: &mut ServiceConfig) {
    config
        .service(all)
        .service(by_disease)
        .service(by_chance)
        .service(new)
        .service(update)
        .service(delete);
}

#[get("/api/chances")]
pub async fn all(pool: web::Data<PgPool>, request: HttpRequest) -> Result<HttpResponse, Error> {
    if request.query_string().is_empty() {
        let items = Model::all(&pool).await.unwrap();
        Ok(HttpResponse::Ok().json(items))
    } else {
        unimplemented!()
    }
}

#[get("/api/chances/{id}")]
pub async fn by_disease(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let item = Model::by_disease(id.into_inner(), &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[get("/api/chances/by-chance/{name}")]
pub async fn by_chance(
    pool: web::Data<PgPool>,
    chance: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let item = Model::by_chance(chance.into_inner(), &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[post("/api/chances")]
pub async fn new(pool: web::Data<PgPool>, item: web::Json<Model>) -> Result<HttpResponse, Error> {
    let item = Model::create(item.into_inner(), &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[patch("/api/chances")]
pub async fn update(
    pool: web::Data<PgPool>,
    item: web::Json<Model>,
) -> Result<HttpResponse, Error> {
    let item = item.into_inner();
    let item = Model::update(item.disease, item.chance, &pool)
        .await
        .unwrap();
    Ok(HttpResponse::Ok().json(item))
}
#[delete("/api/chances/{id}")]
pub async fn delete(pool: web::Data<PgPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let item = Model::delete(id.into_inner(), &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}
