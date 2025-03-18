use actix_web::{HttpResponse, Responder, web};
use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{postgres::Postgres, redis::Redis, schema};

#[derive(Serialize, Selectable, Queryable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::files)]
struct PartialFile {
    id: i32,
    name: String,
}

#[derive(Serialize, Selectable, Queryable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::files)]
struct CompleteFile {
    id: i32,
    name: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}

#[derive(Deserialize)]
pub struct NewFileRequest {
    name: String,
}

#[derive(Insertable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::files)]
struct NewFile {
    name: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct ModifyFileRequest {
    name: String,
}

#[derive(Serialize, AsChangeset)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::files)]
struct ModifyFile {
    name: String,
    updated_at: NaiveDateTime,
}

pub async fn list_files(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut _redis_connection = redis.get_connection();

    let result = schema::files::table
        .select(PartialFile::as_select())
        .load::<PartialFile>(&mut postgres_connection)
        .optional();

    match result {
        Ok(Some(files)) => {
            return HttpResponse::Ok().json(files);
        }
        Ok(None) => {
            return HttpResponse::Ok().json(Vec::<PartialFile>::new());
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn get_file_by_id(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    search_id: web::Path<i32>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut _redis_connection = redis.get_connection();

    let result = schema::files::table
        .select(CompleteFile::as_select())
        .filter(schema::files::id.eq(search_id.into_inner()))
        .first::<CompleteFile>(&mut postgres_connection)
        .optional();

    match result {
        Ok(Some(file)) => {
            return HttpResponse::Ok().json(file);
        }
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn get_file_by_name(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    search_name: web::Path<String>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut _redis_connection = redis.get_connection();

    let result = schema::files::table
        .select(CompleteFile::as_select())
        .filter(schema::files::name.eq(&search_name.into_inner()))
        .first::<CompleteFile>(&mut postgres_connection)
        .optional();

    match result {
        Ok(Some(file)) => {
            return HttpResponse::Ok().json(file);
        }
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn create_file(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    create_file_request: web::Json<NewFileRequest>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut _redis_connection = redis.get_connection();

    let create_file = NewFile {
        name: create_file_request.name.clone(),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let count = diesel::insert_into(schema::files::table)
        .values(&create_file)
        .execute(&mut postgres_connection)
        .optional();

    match count {
        Ok(Some(_)) => {
            return HttpResponse::Created().finish();
        }
        Ok(None) => {
            return HttpResponse::InternalServerError().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn update_file(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    update_id: web::Path<i32>,
    update_file_request: web::Json<ModifyFileRequest>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut _redis_connection = redis.get_connection();

    let update_file = ModifyFile {
        name: update_file_request.name.clone(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let count = diesel::update(schema::files::table)
        .set(&update_file)
        .filter(schema::files::id.eq(update_id.into_inner()))
        .execute(&mut postgres_connection)
        .optional();

    match count {
        Ok(Some(_)) => {
            return HttpResponse::Ok().finish();
        }
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn delete_file(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    delete_id: web::Path<i32>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut _redis_connection = redis.get_connection();

    let count = diesel::delete(schema::files::table)
        .filter(schema::files::id.eq(delete_id.into_inner()))
        .returning(CompleteFile::as_select())
        .execute(&mut postgres_connection)
        .optional();

    match count {
        Ok(Some(_)) => {
            return HttpResponse::Ok().finish();
        }
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}
