use crate::models::{NewSymptom as NewModel, Symptom as Model};
use actix_web::web::ServiceConfig;
use actix_web::{delete, get, patch, post, web, Error, HttpRequest, HttpResponse};
use sqlx::PgPool;

pub fn endpoints(config: &mut ServiceConfig) {
    config
        .service(all)
        .service(by_id)
        .service(by_name)
        .service(new)
        .service(update)
        .service(delete);
}

#[get("/api/symptoms")]
pub async fn all(pool: web::Data<PgPool>, request: HttpRequest) -> Result<HttpResponse, Error> {
    if request.query_string().is_empty() {
        let items = Model::all(&pool).await.unwrap();
        Ok(HttpResponse::Ok().json(items))
    } else {
        unimplemented!()
    }
}

#[get("/api/symptoms/{id}")]
pub async fn by_id(pool: web::Data<PgPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let item = Model::by_id(id.into_inner(), &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[get("/api/symptoms/by-name/{name}")]
pub async fn by_name(
    pool: web::Data<PgPool>,
    name: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let item = Model::by_name(name.into_inner(), &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[post("/api/symptoms")]
pub async fn new(
    pool: web::Data<PgPool>,
    item: web::Json<NewModel>,
) -> Result<HttpResponse, Error> {
    let item = Model::create(item.into_inner(), &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[patch("/api/symptoms/{id}")]
pub async fn update(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
    item: web::Json<NewModel>,
) -> Result<HttpResponse, Error> {
    let item = Model::update(id.into_inner(), item.into_inner(), &pool)
        .await
        .unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[delete("/api/symptoms/{id}")]
pub async fn delete(pool: web::Data<PgPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let item = Model::delete(id.into_inner(), &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}
