use crate::db::establish_connection;
use crate::models::schema::auth_users;
use crate::models::user::User;
use crate::models::request::Request;
use crate::models::claims::Claims;
use bcrypt::hash;
use diesel::prelude::*;
use dotenv::dotenv;
use jsonwebtoken as jwt;
use std::env;

pub async fn signup(info: Request) -> String {
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

    let my_claims = Claims {
        sub: new_user.id.to_owned(),
        iat: jwt::get_current_timestamp(),
        exp: jwt::get_current_timestamp() + (60 * 60 * 24 * 14)
    };

    // insert new user to database
    diesel::insert_into(auth_users::table)
        .values(&new_user)
        .execute(&mut connection)
        .expect("Error saving new user");

    // create jwt token from user
    let jwt_token = jwt::encode(
        &jwt::Header::new(jwt::Algorithm::HS256),
        &my_claims,
        // &new_user.id.to_string(),
        &jwt::EncodingKey::from_secret(access_token_secret.as_bytes()),
    );

    // return jwt token
    return jwt_token.unwrap();
}
