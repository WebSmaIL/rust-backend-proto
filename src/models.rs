use serde::{Deserialize, Serialize};

use crate::schema::users;

/// User details.
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub login: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct FormattedUser {
    pub id: String,
    pub email: String,
    pub login: String,
}

impl User {
    pub fn format_user(&self) -> FormattedUser {
        let formated_user: FormattedUser = FormattedUser {
            id: self.id.clone(),
            login: self.login.clone(),
            email: self.email.clone(),
        };
        return formated_user;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginUser {
    pub password: String,
    pub login: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub password: String,
    pub email: String,
    pub login: String,
}
