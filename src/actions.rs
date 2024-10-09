use std::{f64::consts::PI, fmt::UpperHex};

use actix_web::web;
use diesel::prelude::*;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::models::{self, FormattedUser};

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_user_by_uid(
    conn: &mut PgConnection,
    uid: Uuid,
) -> Result<Option<models::User>, DbError> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(id.eq(uid.to_string()))
        .first::<models::User>(conn)
        .optional()?;

    Ok(user)
}

pub fn insert_new_user(
    conn: &mut PgConnection,
    user: &web::Json<models::NewUser>,
) -> Result<models::User, DbError> {
    use crate::schema::users::dsl::*;
    let hash = Sha256::digest(user.password.clone());

    let string_hash = format!("{:X}", hash);
    println!("{}", string_hash);

    let new_user = models::User {
        id: Uuid::new_v4().to_string(),
        login: user.login.clone(),
        password: string_hash,
        email: user.email.clone(),
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(new_user)
}

pub fn check_user(
    conn: &mut PgConnection,
    user: &web::Json<models::LoginUser>,
) -> Result<Result<FormattedUser, String>, DbError> {
    use crate::schema::users::dsl::*;

    let user_db: Option<models::User> = users
        .filter(login.eq(user.login.clone()))
        .first::<models::User>(conn)
        .optional()?;

    let result = match user_db {
        Some(user_db) => {
            let hash = Sha256::digest(user.password.clone());
            let string_hash = format!("{:X}", hash);
            let is_true_pass = user_db.password == string_hash;
            println!("request hash {}", string_hash);
            println!("db hash {:?}", user_db.password);

            let is_true_result = match is_true_pass {
                true => Ok(user_db.format_user()),
                false => Err("User password is incorrectly".to_string()),
            };
            is_true_result
        }
        None => Err("User Not Found".to_string()),
    };

    return Ok(result);
}
