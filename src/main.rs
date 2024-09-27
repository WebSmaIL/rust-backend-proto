#[macro_use]
extern crate diesel;

use actix_web::{error, get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::{prelude::*, r2d2};
use uuid::Uuid;

mod actions;
mod models;
mod schema;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

#[get("/user/{user_id}")]
async fn get_user(
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
async fn add_user(
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // initialize DB pool outside of `HttpServer::new` so that it is shared across all workers
    let pool = initialize_db_pool();

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(get_user)
            .service(add_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn initialize_db_pool() -> DbPool {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(conn_spec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to Postgres DB file")
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};

    use super::*;

    #[actix_web::test]
    async fn user_routes() {
        dotenvy::dotenv().ok();
        env_logger::try_init_from_env(env_logger::Env::new().default_filter_or("info")).ok();

        let pool = initialize_db_pool();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .wrap(middleware::Logger::default())
                .service(get_user)
                .service(add_user),
        )
        .await;

        let req = test::TestRequest::get().uri("/user/123").to_request();
        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
        let body = test::read_body(res).await;
        assert!(
            body.starts_with(b"UUID parsing failed"),
            "unexpected body: {body:?}",
        );

        // try to find a non-existent user
        let req = test::TestRequest::get()
            .uri(&format!("/user/{}", Uuid::nil()))
            .to_request();
        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
        let body = test::read_body(res).await;
        assert!(
            body.starts_with(b"No user found"),
            "unexpected body: {body:?}",
        );

        let req = test::TestRequest::post()
            .uri("/user")
            .set_json(models::NewUser::new("Test user"))
            .to_request();
        let res: models::User = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.name, "Test user");

        let req = test::TestRequest::get()
            .uri(&format!("/user/{}", res.id))
            .to_request();
        let res: models::User = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.name, "Test user");

        use crate::schema::users::dsl::*;
        diesel::delete(users.filter(id.eq(res.id)))
            .execute(&mut pool.get().expect("couldn't get db connection from pool"))
            .expect("couldn't delete test user from table");
    }
}