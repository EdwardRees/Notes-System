use crate::db::establish_connection;
use crate::models::schema::auth_users;
use crate::models::user::User;
use bcrypt::hash;
use diesel::prelude::*;
use dotenv::dotenv;
use jsonwebtoken as jwt;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct SignupRequest {
    email: String,
    password: String,
}

pub async fn signup(info: SignupRequest) -> String {
    dotenv().ok();
    let email: &String = &info.email;
    let password: &String = &info.password;
    let hashed_password: String = hash(password, 10).unwrap();
    let access_token_secret =
        env::var("ACCESS_TOKEN_SECRET").expect("ACCESS_TOKEN_SECRET must be set");
    let mut connection = establish_connection();
    // create new user
    let new_user = User {
        id: uuid::Uuid::new_v4().to_string(),
        email: email.to_string(),
        password: hashed_password,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };

    // insert new user to database
    diesel::insert_into(auth_users::table)
        .values(&new_user)
        .execute(&mut connection)
        .expect("Error saving new user");

    // create jwt token from user
    let jwt_token = jwt::encode(
        &jwt::Header::new(jwt::Algorithm::HS256),
        &new_user.id.to_string(),
        &jwt::EncodingKey::from_secret(access_token_secret.as_bytes()),
    );

    // return jwt token
    return jwt_token.unwrap();
}
