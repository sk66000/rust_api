use actix_web::{web, get, post, put, delete, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::{Item, CreateItem, UpdateItem};

#[post("/items")]
pub async fn create_item(pool: web::Data <PgPool>, item: web::Json <CreateItem>) -> impl Responder {
    match sqlx::query_as!(
        Item,
        "INSERT INTO items (name, description) VALUES ($1, $2) RETURNING id, name, description",
        item.name,
        item.description
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/items/{id}")]
pub async fn get_item(pool: web::Data <PgPool>, id: web::Path <i32>) -> impl Responder {
    match sqlx::query_as!(Item, "SELECT * FROM items WHERE id = $1", id.into_inner())
        .fetch_optional(pool.get_ref())
        .await
    {
        Ok(Some(item)) => HttpResponse::Ok().json(item),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[put("/items/{id}")]
pub async fn update_item(
    pool: web::Data <PgPool>,
    id: web::Path <i32>,
    item: web::Json <UpdateItem>,
) -> impl Responder {

    match sqlx::query_as!(
        Item,
        "UPDATE items SET name = COALESCE($1, name), description = COALESCE($2, description) WHERE id = $3 RETURNING id, name, description",
        item.name,
        item.description,
        id.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await
    {
        Ok(Some(item)) => HttpResponse::Ok().json(item),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/items/{id}")]
pub async fn delete_item(pool: web::Data <PgPool>, id: web::Path <i32>) -> impl Responder {

    match sqlx::query!("DELETE FROM items WHERE id = $1", id.into_inner())
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
