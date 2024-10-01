use actix_web::delete;
#[macro_use]
use actix_web::{error, get, post, web, HttpResponse, Responder};
use uuid::Uuid;

use crate::{actions, models, DbPool};

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

#[post("/user")]
pub async fn add_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUser>,
) -> actix_web::Result<impl Responder> {
    // use web::block to offload blocking Diesel queries without blocking server thread
    let user = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::insert_new_user(&mut conn, &form.name)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    // user was added successfully; return 201 response with new user info
    Ok(HttpResponse::Created().json(user))
}


#[delete("/user/{user_id}")]
pub async fn del_user(
    pool: web::Data<DbPool>,
    user_uid: web::Path<Uuid>,
) -> {

}
