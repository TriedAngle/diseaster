use crate::models::{Department as Model, NewDepartment as NewModel};
use actix_web::web::ServiceConfig;
use actix_web::{delete, get, patch, post, web, Error, HttpRequest, HttpResponse};
use sqlx::PgPool;

pub fn endpoints(config: &mut ServiceConfig) {
    config
        .service(all)
        .service(by_id)
        .service(by_name)
        .service(by_disease)
        .service(new)
        .service(update)
        .service(delete);
}

#[get("/api/departments")]
pub async fn all(pool: web::Data<PgPool>, request: HttpRequest) -> Result<HttpResponse, Error> {
    if request.query_string().is_empty() {
        let items = Model::all(&pool).await.unwrap();
        Ok(HttpResponse::Ok().json(items))
    } else {
        unimplemented!()
    }
}

#[get("/api/departments/{id}")]
pub async fn by_id(pool: web::Data<PgPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let item = Model::by_id(id.into_inner(), &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[get("/api/departments/by-name/{name}")]
pub async fn by_name(
    pool: web::Data<PgPool>,
    name: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let item = Model::by_name(name.into_inner(), &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}

// #[get("/api/departments/by-diseases")]
// pub async fn by_diseases(
//     pool: web::Data<PgPool>,
//     diseases: web::Path<Vec<i32>>,
// ) -> Result<HttpResponse, Error> {
//     let item = Model::by_diseases(&diseases.into_inner(), &pool)
//         .await
//         .unwrap();
//     Ok(HttpResponse::Ok().json(item))
// }

#[get("/api/departments/by-disease/{disease}")]
pub async fn by_disease(
    pool: web::Data<PgPool>,
    disease: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let diseases = vec![disease.into_inner()];
    let item = Model::by_disease(&diseases, &pool)
        .await
        .unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[post("/api/departments")]
pub async fn new(
    pool: web::Data<PgPool>,
    item: web::Json<NewModel>,
) -> Result<HttpResponse, Error> {
    let item = Model::create(item.into_inner(), &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[patch("/api/departments/{id}")]
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
#[delete("/api/departments/{id}")]
pub async fn delete(pool: web::Data<PgPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let item = Model::delete(id.into_inner(), &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}
