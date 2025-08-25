use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub age: i32,
}
