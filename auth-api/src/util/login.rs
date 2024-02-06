use crate::db::establish_connection;
use crate::models::schema::auth_users;
use crate::models::user::User;
use crate::models::request::Request;
use crate::models::claims::Claims;
use bcrypt::verify;
use diesel::prelude::*;
use dotenv::dotenv;
use jsonwebtoken as jwt;
use std::env;

pub async fn login(info: Request) -> String {
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
    };

    let my_claims = Claims {
        sub: user.id.to_owned(),
        iat: jwt::get_current_timestamp(),
        exp: jwt::get_current_timestamp() + (60 * 60 * 24 * 14)
    };

    let jwt_token = jwt::encode(
        &jwt::Header::new(jwt::Algorithm::HS256),
        &my_claims,
        //&user.id.to_string(),
        &jwt::EncodingKey::from_secret(access_token_secret.as_bytes()),
    );

    return jwt_token.unwrap();
}
