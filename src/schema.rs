// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Varchar,
        login -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}
