use serde::{Deserialize, Serialize};
use super::schema::users;
use uuid::Uuid;
use diesel::Queryable;
use diesel::Insertable;
use diesel::Selectable;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
pub struct User {
    #[serde(with = "uuid::serde::compact")]
    pub id: Uuid,
    pub name: String,
    // ... другие поля ...
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
}