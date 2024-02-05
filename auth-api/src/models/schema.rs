// @generated automatically by Diesel CLI.

diesel::table! {
    auth_users (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
