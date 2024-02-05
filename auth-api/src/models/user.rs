use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::models::schema::auth_users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
  pub id: String,
  pub email: String,
  pub password: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime
}
