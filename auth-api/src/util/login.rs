use crate::db::establish_connection;
use crate::models::schema::auth_users;
use crate::models::user::User;
use bcrypt::verify;
use diesel::prelude::*;
use dotenv::dotenv;
use jsonwebtoken as jwt;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

pub async fn login(info: LoginRequest) -> String {
    dotenv().ok();
    let email: &String = &info.email;
    let password: &String = &info.password;
    let access_token_secret =
        env::var("ACCESS_TOKEN_SECRET").expect("ACCESS_TOKEN_SECRET must be set");
    let mut connection = establish_connection();
    let user = auth_users::table
        .filter(auth_users::email.eq(email))
        .first::<User>(&mut connection)
        .expect("Error finding user");

    let is_valid = verify(password, &user.password).unwrap();

    if !is_valid {
        return "Invalid password".to_string();
    }

    let jwt_token = jwt::encode(
        &jwt::Header::new(jwt::Algorithm::HS256),
        &user.id.to_string(),
        &jwt::EncodingKey::from_secret(access_token_secret.as_bytes()),
    );

    return jwt_token.unwrap();
}
