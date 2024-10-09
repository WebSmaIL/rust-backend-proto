use actix_web::{error, get, post, web, HttpResponse, Responder};
use uuid::Uuid;

use crate::{
    actions,
    models::{self, FormattedUser},
    DbPool,
};

// get method for get user by id
#[get("/user/{user_id}")]
pub async fn get_user(
    pool: web::Data<DbPool>,
    user_uid: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let user_uid = user_uid.into_inner();

    let user = web::block(move || {
        let mut conn = pool.get()?;

        actions::find_user_by_uid(&mut conn, user_uid)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body(format!("No user found with UID: {user_uid}")),
    })
}

// post method for insert user using body
#[post("/user")]
pub async fn add_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUser>,
) -> actix_web::Result<impl Responder> {
    let user = web::block(move || {
        let mut conn = pool.get()?;

        actions::insert_new_user(&mut conn, &form)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?
    .format_user();

    Ok(HttpResponse::Created().json(user))
}

// post method for insert user using body: name
#[post("/login")]
pub async fn login_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::LoginUser>,
) -> actix_web::Result<impl Responder> {
    let result = web::block(move || {
        let mut conn = pool.get()?;
        actions::check_user(&mut conn, &form)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(result) => HttpResponse::BadRequest().json(result),
    })
}
